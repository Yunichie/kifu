use std::str::FromStr;

use percent_encoding::percent_decode_str;
use roxmltree::Node;

use super::{calls::decode_call, rules::decode_rules, yaku::yaku_name};
use crate::{
    DomainError, Result,
    types::{DrawReason, Kyoku, KyokuResult, Ruleset, TurnEvent, WinResult, Yaku},
};

const RANK_NAMES: [&str; 21] = [
    "Novice", "9kyu", "8kyu", "7kyu", "6kyu", "5kyu", "4kyu", "3kyu", "2kyu", "1kyu", "1dan",
    "2dan", "3dan", "4dan", "5dan", "6dan", "7dan", "8dan", "9dan", "10dan", "Tenhoui",
];

type ParsedDraw = (DrawReason, Vec<u8>, Vec<i32>, Vec<i32>);

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedPlayer {
    pub seat: u8,
    pub name: String,
    pub rank_id: u8,
    pub rank: String,
    pub rating: f64,
    pub sex: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedGame {
    pub rules: Ruleset,
    pub players: Vec<ParsedPlayer>,
    pub kyoku: Vec<Kyoku>,
    pub final_scores: Option<Vec<i32>>,
    pub uma: Option<Vec<f64>>,
}

enum PendingResult {
    None,
    Wins(Vec<WinResult>),
    Draw {
        reason: DrawReason,
        tenpai_seats: Vec<u8>,
        score_deltas: Vec<i32>,
    },
}

struct KyokuBuilder {
    round_index: u8,
    bakaze: u8,
    kyoku_number: u8,
    honba: u8,
    riichi_sticks: u8,
    dealer: u8,
    start_scores: Vec<i32>,
    end_scores: Vec<i32>,
    start_hands: Vec<Vec<u16>>,
    dora_indicators: Vec<u16>,
    events: Vec<TurnEvent>,
    last_draw: [Option<u16>; 4],
    result: PendingResult,
}

impl KyokuBuilder {
    fn from_init(node: Node<'_, '_>) -> Result<Self> {
        let seed = parse_csv::<u16>(node, "seed")?;
        if seed.len() != 6
            || seed[..3].iter().any(|value| *value > u16::from(u8::MAX))
            || seed[5] > 135
        {
            return Err(invalid_attribute(node, "seed"));
        }

        let start_scores = parse_csv::<i32>(node, "ten")?
            .into_iter()
            .map(|score| to_points(node, "ten", score))
            .collect::<Result<Vec<_>>>()?;
        if start_scores.len() != 4 {
            return Err(invalid_attribute(node, "ten"));
        }

        let mut start_hands = Vec::with_capacity(4);
        for seat in 0..4 {
            let attribute = format!("hai{seat}");
            let hand = parse_tile_csv(node, &attribute)?;
            start_hands.push(hand);
        }

        let round_index = seed[0] as u8;
        Ok(Self {
            round_index,
            bakaze: round_index / 4,
            kyoku_number: round_index % 4 + 1,
            honba: seed[1] as u8,
            riichi_sticks: seed[2] as u8,
            dealer: parse_seat_attr(node, "oya")?,
            end_scores: start_scores.clone(),
            start_scores,
            start_hands,
            dora_indicators: vec![seed[5]],
            events: Vec::new(),
            last_draw: [None; 4],
            result: PendingResult::None,
        })
    }

    fn push_win(&mut self, win: WinResult, end_scores: Vec<i32>) -> Result<()> {
        match &mut self.result {
            PendingResult::None => self.result = PendingResult::Wins(vec![win]),
            PendingResult::Wins(wins) => wins.push(win),
            PendingResult::Draw { .. } => {
                return Err(DomainError::ConflictingResult {
                    round: self.round_index,
                });
            }
        }
        self.end_scores = end_scores;
        Ok(())
    }

    fn set_draw(
        &mut self,
        reason: DrawReason,
        tenpai_seats: Vec<u8>,
        score_deltas: Vec<i32>,
        end_scores: Vec<i32>,
    ) -> Result<()> {
        if !matches!(self.result, PendingResult::None) {
            return Err(DomainError::ConflictingResult {
                round: self.round_index,
            });
        }
        self.result = PendingResult::Draw {
            reason,
            tenpai_seats,
            score_deltas,
        };
        self.end_scores = end_scores;
        Ok(())
    }

    fn finish(self) -> Result<Kyoku> {
        let result = match self.result {
            PendingResult::None => {
                return Err(DomainError::IncompleteKyoku {
                    round: self.round_index,
                });
            }
            PendingResult::Wins(wins) => KyokuResult::Win { wins },
            PendingResult::Draw {
                reason,
                tenpai_seats,
                score_deltas,
            } => KyokuResult::Draw {
                reason,
                tenpai_seats,
                score_deltas,
            },
        };

        Ok(Kyoku {
            round_index: self.round_index,
            bakaze: self.bakaze,
            kyoku_number: self.kyoku_number,
            honba: self.honba,
            riichi_sticks: self.riichi_sticks,
            dealer: self.dealer,
            start_scores: self.start_scores,
            end_scores: self.end_scores,
            start_hands: self.start_hands,
            dora_indicators: self.dora_indicators,
            events: self.events,
            result,
        })
    }
}

pub fn parse_mjlog(xml: &str) -> Result<ParsedGame> {
    let document = roxmltree::Document::parse(xml)?;
    let root = document.root_element();
    if root.tag_name().name() != "mjloggm" {
        return Err(DomainError::MissingMetadata("mjloggm root"));
    }

    let mut rules = None;
    let mut players = None;
    let mut kyoku = Vec::new();
    let mut current = None;
    let mut final_scores = None;
    let mut uma = None;

    for node in root.children().filter(Node::is_element) {
        let tag = node.tag_name().name();
        match tag {
            "GO" => {
                rules = Some(decode_rules(
                    parse_attr(node, "type")?,
                    node.attribute("lobby").unwrap_or("0").to_owned(),
                ));
            }
            "UN" if players.is_none() => players = Some(parse_players(node)?),
            "INIT" => {
                if let Some(previous) = current.take() {
                    kyoku.push(KyokuBuilder::finish(previous)?);
                }
                current = Some(KyokuBuilder::from_init(node)?);
            }
            "N" => {
                let builder = current_builder(&mut current)?;
                let seat = parse_seat_attr(node, "who")?;
                let encoded = parse_attr(node, "m")?;
                let call = decode_call(seat, encoded);
                if call.tiles.iter().any(|tile| *tile > 135) {
                    return Err(invalid_attribute(node, "m"));
                }
                builder.events.push(TurnEvent::Call {
                    seat,
                    kind: call.kind,
                    tiles: call.tiles,
                    from_seat: call.from_seat,
                });
            }
            "REACH" if node.attribute("step") == Some("1") => {
                current_builder(&mut current)?
                    .events
                    .push(TurnEvent::Riichi {
                        seat: parse_seat_attr(node, "who")?,
                    });
            }
            "DORA" => {
                let tile = parse_tile_attr(node, "hai")?;
                let builder = current_builder(&mut current)?;
                builder.dora_indicators.push(tile);
                builder.events.push(TurnEvent::NewDora { tile });
            }
            "AGARI" => {
                let (win, end_scores) = parse_win(node)?;
                current_builder(&mut current)?.push_win(win, end_scores)?;
                if let Some((scores, game_uma)) = parse_owari(node)? {
                    final_scores = Some(scores);
                    uma = Some(game_uma);
                }
            }
            "RYUUKYOKU" => {
                let (reason, tenpai_seats, score_deltas, end_scores) = parse_draw(node)?;
                current_builder(&mut current)?.set_draw(
                    reason,
                    tenpai_seats,
                    score_deltas,
                    end_scores,
                )?;
                if let Some((scores, game_uma)) = parse_owari(node)? {
                    final_scores = Some(scores);
                    uma = Some(game_uma);
                }
            }
            _ => {
                if let Some((draw, seat, tile)) = parse_turn_tag(tag)? {
                    let builder = current_builder(&mut current)?;
                    if draw {
                        builder.last_draw[usize::from(seat)] = Some(tile);
                        builder.events.push(TurnEvent::Draw { seat, tile });
                    } else {
                        let tsumogiri = builder.last_draw[usize::from(seat)] == Some(tile);
                        builder.last_draw[usize::from(seat)] = None;
                        builder.events.push(TurnEvent::Discard {
                            seat,
                            tile,
                            tsumogiri,
                        });
                    }
                }
            }
        }
    }

    if let Some(last) = current {
        kyoku.push(last.finish()?);
    }
    if kyoku.is_empty() {
        return Err(DomainError::MissingMetadata("kyoku"));
    }

    Ok(ParsedGame {
        rules: rules.ok_or(DomainError::MissingMetadata("GO"))?,
        players: players.ok_or(DomainError::MissingMetadata("UN"))?,
        kyoku,
        final_scores,
        uma,
    })
}

fn parse_players(node: Node<'_, '_>) -> Result<Vec<ParsedPlayer>> {
    let ranks = parse_csv::<u8>(node, "dan")?;
    let ratings = parse_csv::<f64>(node, "rate")?;
    let sexes = required_attr(node, "sx")?.split(',').collect::<Vec<_>>();
    if ranks.len() != 4 || ratings.len() != 4 || sexes.len() != 4 {
        return Err(invalid_attribute(node, "dan/rate/sx"));
    }

    (0..4)
        .map(|seat| {
            let attribute = format!("n{seat}");
            let encoded = node.attribute(attribute.as_str()).unwrap_or_default();
            let name = percent_decode_str(encoded)
                .decode_utf8()
                .map_err(|_| invalid_attribute(node, &attribute))?
                .into_owned();
            let rank_id = ranks[seat];
            let rank = RANK_NAMES
                .get(usize::from(rank_id))
                .copied()
                .unwrap_or("Unknown")
                .to_owned();
            Ok(ParsedPlayer {
                seat: seat as u8,
                name,
                rank_id,
                rank,
                rating: ratings[seat],
                sex: sexes[seat].to_owned(),
            })
        })
        .collect()
}

fn parse_win(node: Node<'_, '_>) -> Result<(WinResult, Vec<i32>)> {
    let winner = parse_seat_attr(node, "who")?;
    let from_seat = parse_seat_attr(node, "fromWho")?;
    let ten = parse_csv::<u32>(node, "ten")?;
    if ten.len() != 3 || ten[0] > u32::from(u16::MAX) || ten[2] > u32::from(u8::MAX) {
        return Err(invalid_attribute(node, "ten"));
    }

    let mut yaku = Vec::new();
    if node.attribute("yaku").is_some() {
        let values = parse_csv::<u8>(node, "yaku")?;
        if values.len() % 2 != 0 {
            return Err(invalid_attribute(node, "yaku"));
        }
        yaku.extend(values.chunks_exact(2).map(|pair| Yaku {
            id: pair[0],
            name: yaku_name(pair[0]),
            han: pair[1],
            yakuman: false,
        }));
    }
    if node.attribute("yakuman").is_some() {
        yaku.extend(
            parse_csv::<u8>(node, "yakuman")?
                .into_iter()
                .map(|id| Yaku {
                    id,
                    name: yaku_name(id),
                    han: 13,
                    yakuman: true,
                }),
        );
    }

    let (scores_before, score_deltas) = parse_score_changes(node)?;
    let end_scores = scores_before
        .iter()
        .zip(&score_deltas)
        .map(|(before, delta)| {
            before
                .checked_add(*delta)
                .ok_or_else(|| invalid_attribute(node, "sc"))
        })
        .collect::<Result<Vec<_>>>()?;
    let han = yaku
        .iter()
        .map(|entry| u16::from(entry.han))
        .sum::<u16>()
        .min(u16::from(u8::MAX)) as u8;

    Ok((
        WinResult {
            winner,
            from_seat,
            tsumo: winner == from_seat,
            fu: ten[0] as u16,
            han,
            points: ten[1],
            limit: ten[2] as u8,
            winning_tiles: parse_tile_csv(node, "hai")?,
            wait: parse_tile_attr(node, "machi")?,
            dora_indicators: parse_optional_tile_csv(node, "doraHai")?,
            yaku,
            score_deltas,
        },
        end_scores,
    ))
}

fn parse_draw(node: Node<'_, '_>) -> Result<ParsedDraw> {
    let reason = match node.attribute("type").unwrap_or_default() {
        "" => DrawReason::Exhaustive,
        "yao9" => DrawReason::NineTerminals,
        "reach4" => DrawReason::FourRiichi,
        "ron3" => DrawReason::TripleRon,
        "kan4" => DrawReason::FourKans,
        "kaze4" => DrawReason::FourWinds,
        "nm" | "nm4" => DrawReason::NagashiMangan,
        code => DrawReason::Other {
            code: code.to_owned(),
        },
    };
    let tenpai_seats = (0..4)
        .filter(|seat| node.attribute(format!("hai{seat}").as_str()).is_some())
        .map(|seat| seat as u8)
        .collect();
    let (scores_before, score_deltas) = parse_score_changes(node)?;
    let end_scores = scores_before
        .iter()
        .zip(&score_deltas)
        .map(|(before, delta)| {
            before
                .checked_add(*delta)
                .ok_or_else(|| invalid_attribute(node, "sc"))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok((reason, tenpai_seats, score_deltas, end_scores))
}

fn parse_score_changes(node: Node<'_, '_>) -> Result<(Vec<i32>, Vec<i32>)> {
    let values = parse_csv::<i32>(node, "sc")?;
    if values.len() != 8 {
        return Err(invalid_attribute(node, "sc"));
    }
    let scores_before = values
        .iter()
        .step_by(2)
        .map(|value| to_points(node, "sc", *value))
        .collect::<Result<Vec<_>>>()?;
    let deltas = values
        .iter()
        .skip(1)
        .step_by(2)
        .map(|value| to_points(node, "sc", *value))
        .collect::<Result<Vec<_>>>()?;
    Ok((scores_before, deltas))
}

fn parse_owari(node: Node<'_, '_>) -> Result<Option<(Vec<i32>, Vec<f64>)>> {
    let Some(value) = node.attribute("owari") else {
        return Ok(None);
    };
    let values = value.split(',').collect::<Vec<_>>();
    if values.len() != 8 {
        return Err(invalid_attribute(node, "owari"));
    }

    let mut scores = Vec::with_capacity(4);
    let mut uma = Vec::with_capacity(4);
    for pair in values.chunks_exact(2) {
        let score = pair[0]
            .parse::<i32>()
            .map_err(|_| invalid_attribute(node, "owari"))?;
        scores.push(to_points(node, "owari", score)?);
        uma.push(
            pair[1]
                .parse::<f64>()
                .map_err(|_| invalid_attribute(node, "owari"))?,
        );
    }
    Ok(Some((scores, uma)))
}

fn parse_turn_tag(tag: &str) -> Result<Option<(bool, u8, u16)>> {
    let (prefix, tile) = tag.split_at(1);
    if tile.is_empty() || !tile.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(None);
    }
    let (draw, seat) = match prefix {
        "T" => (true, 0),
        "U" => (true, 1),
        "V" => (true, 2),
        "W" => (true, 3),
        "D" => (false, 0),
        "E" => (false, 1),
        "F" => (false, 2),
        "G" => (false, 3),
        _ => return Ok(None),
    };
    let tile = tile
        .parse::<u16>()
        .map_err(|_| DomainError::InvalidAttribute {
            tag: tag.to_owned(),
            attribute: "tile".to_owned(),
            value: tile.to_owned(),
        })?;
    if tile > 135 {
        return Err(DomainError::InvalidAttribute {
            tag: tag.to_owned(),
            attribute: "tile".to_owned(),
            value: tile.to_string(),
        });
    }
    Ok(Some((draw, seat, tile)))
}

fn current_builder(current: &mut Option<KyokuBuilder>) -> Result<&mut KyokuBuilder> {
    current
        .as_mut()
        .ok_or(DomainError::MissingMetadata("INIT before turn data"))
}

fn parse_tile_attr(node: Node<'_, '_>, attribute: &str) -> Result<u16> {
    let tile = parse_attr(node, attribute)?;
    if tile > 135 {
        return Err(invalid_attribute(node, attribute));
    }
    Ok(tile)
}

fn parse_seat_attr(node: Node<'_, '_>, attribute: &str) -> Result<u8> {
    let seat = parse_attr(node, attribute)?;
    if seat > 3 {
        return Err(invalid_attribute(node, attribute));
    }
    Ok(seat)
}

fn parse_tile_csv(node: Node<'_, '_>, attribute: &str) -> Result<Vec<u16>> {
    let tiles = parse_csv::<u16>(node, attribute)?;
    if tiles.iter().any(|tile| *tile > 135) {
        return Err(invalid_attribute(node, attribute));
    }
    Ok(tiles)
}

fn parse_optional_tile_csv(node: Node<'_, '_>, attribute: &str) -> Result<Vec<u16>> {
    if node.attribute(attribute).is_none() {
        return Ok(Vec::new());
    }
    parse_tile_csv(node, attribute)
}

fn parse_attr<T>(node: Node<'_, '_>, attribute: &str) -> Result<T>
where
    T: FromStr,
{
    required_attr(node, attribute)?
        .parse()
        .map_err(|_| invalid_attribute(node, attribute))
}

fn parse_csv<T>(node: Node<'_, '_>, attribute: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    let value = required_attr(node, attribute)?;
    if value.is_empty() {
        return Ok(Vec::new());
    }
    value
        .split(',')
        .map(|part| part.parse().map_err(|_| invalid_attribute(node, attribute)))
        .collect()
}

fn required_attr<'a>(node: Node<'a, '_>, attribute: &str) -> Result<&'a str> {
    node.attribute(attribute)
        .ok_or_else(|| DomainError::MissingAttribute {
            tag: node.tag_name().name().to_owned(),
            attribute: attribute.to_owned(),
        })
}

fn invalid_attribute(node: Node<'_, '_>, attribute: &str) -> DomainError {
    DomainError::InvalidAttribute {
        tag: node.tag_name().name().to_owned(),
        attribute: attribute.to_owned(),
        value: node.attribute(attribute).unwrap_or_default().to_owned(),
    }
}

fn to_points(node: Node<'_, '_>, attribute: &str, value: i32) -> Result<i32> {
    value
        .checked_mul(100)
        .ok_or_else(|| invalid_attribute(node, attribute))
}

#[cfg(test)]
mod tests {
    use super::parse_mjlog;
    use crate::types::{DrawReason, KyokuResult, TurnEvent};

    #[test]
    fn retains_new_dora_in_event_order() {
        let xml = r#"<mjloggm ver="2.3"><GO type="169" lobby="0"/><UN n0="A" n1="B" n2="C" n3="D" dan="0,0,0,0" rate="1500,1500,1500,1500" sx="M,M,M,M"/><INIT seed="0,0,0,1,1,4" ten="250,250,250,250" oya="0" hai0="0" hai1="1" hai2="2" hai3="3"/><DORA hai="8"/><RYUUKYOKU type="yao9" sc="250,0,250,0,250,0,250,0"/></mjloggm>"#;
        let parsed = parse_mjlog(xml).unwrap();

        assert_eq!(parsed.kyoku[0].dora_indicators, [4, 8]);
        assert_eq!(parsed.kyoku[0].events, [TurnEvent::NewDora { tile: 8 }]);
        assert!(matches!(
            parsed.kyoku[0].result,
            KyokuResult::Draw {
                reason: DrawReason::NineTerminals,
                ..
            }
        ));
    }

    #[test]
    fn rejects_out_of_range_turn_tiles() {
        let xml = r#"<mjloggm ver="2.3"><GO type="169"/><UN n0="A" n1="B" n2="C" n3="D" dan="0,0,0,0" rate="1500,1500,1500,1500" sx="M,M,M,M"/><INIT seed="0,0,0,1,1,4" ten="250,250,250,250" oya="0" hai0="0" hai1="1" hai2="2" hai3="3"/><T136/><RYUUKYOKU sc="250,0,250,0,250,0,250,0"/></mjloggm>"#;

        assert!(parse_mjlog(xml).is_err());
    }
}
