use std::collections::{BTreeMap, BTreeSet};

use crate::{
    mjlog::parser::ParsedGame,
    types::{
        BonusStats, CallKind, CallStats, CareerStats, CountBucket, DealInMatrix, DrawReason,
        GameDetail, KyokuResult, PlayerRates, PlayerStats, PlayerSummary, ScoreTrendPoint,
        TurnEvent, WinResult, YakuCount,
    },
};

#[derive(Default)]
struct StatsAccumulator {
    stats: PlayerStats,
    yaku: BTreeMap<u8, (String, u32)>,
    han: BTreeMap<u32, u32>,
    fu: BTreeMap<u32, u32>,
    hand_value: BTreeMap<u32, u32>,
}

impl StatsAccumulator {
    fn record_win(&mut self, win: &WinResult) {
        self.stats.wins += 1;
        if win.tsumo {
            self.stats.tsumo_wins += 1;
        } else {
            self.stats.ron_wins += 1;
        }
        increment(&mut self.han, u32::from(win.han), 1);
        increment(&mut self.fu, u32::from(win.fu), 1);
        increment(&mut self.hand_value, win.points, 1);

        let mut has_dora = false;
        let mut has_ura = false;
        let mut has_aka = false;
        for yaku in win.yaku.iter().filter(|yaku| yaku.han > 0) {
            self.yaku
                .entry(yaku.id)
                .and_modify(|(_, count)| *count += 1)
                .or_insert_with(|| (yaku.name.clone(), 1));
            match yaku.id {
                52 => {
                    self.stats.bonuses.dora += u32::from(yaku.han);
                    has_dora = true;
                }
                53 => {
                    self.stats.bonuses.ura_dora += u32::from(yaku.han);
                    has_ura = true;
                }
                54 => {
                    self.stats.bonuses.aka_dora += u32::from(yaku.han);
                    has_aka = true;
                }
                _ => {}
            }
        }
        self.stats.bonuses.wins_with_dora += u32::from(has_dora);
        self.stats.bonuses.wins_with_ura_dora += u32::from(has_ura);
        self.stats.bonuses.wins_with_aka_dora += u32::from(has_aka);
    }

    fn record_call(&mut self, kind: CallKind) {
        self.stats.calls.total += 1;
        match kind {
            CallKind::Chi => self.stats.calls.chi += 1,
            CallKind::Pon => self.stats.calls.pon += 1,
            CallKind::OpenKan => self.stats.calls.open_kan += 1,
            CallKind::ClosedKan => self.stats.calls.closed_kan += 1,
            CallKind::AddedKan => self.stats.calls.added_kan += 1,
            CallKind::Nuki => self.stats.calls.nuki += 1,
        }
    }

    fn merge(&mut self, stats: &PlayerStats) {
        self.stats.hands += stats.hands;
        self.stats.wins += stats.wins;
        self.stats.tsumo_wins += stats.tsumo_wins;
        self.stats.ron_wins += stats.ron_wins;
        self.stats.deal_ins += stats.deal_ins;
        self.stats.riichi += stats.riichi;
        self.stats.dealer_hands += stats.dealer_hands;
        self.stats.dealer_repeats += stats.dealer_repeats;
        self.stats.exhaustive_draws += stats.exhaustive_draws;
        self.stats.tenpai_draws += stats.tenpai_draws;
        self.stats.noten_draws += stats.noten_draws;
        self.stats.called_hands += stats.called_hands;
        merge_calls(&mut self.stats.calls, &stats.calls);
        merge_bonuses(&mut self.stats.bonuses, &stats.bonuses);
        for yaku in &stats.yaku_frequency {
            self.yaku
                .entry(yaku.id)
                .and_modify(|(_, count)| *count += yaku.count)
                .or_insert_with(|| (yaku.name.clone(), yaku.count));
        }
        merge_buckets(&mut self.han, &stats.han_distribution);
        merge_buckets(&mut self.fu, &stats.fu_distribution);
        merge_buckets(&mut self.hand_value, &stats.hand_value_distribution);
    }

    fn finish(mut self) -> PlayerStats {
        let hands = self.stats.hands;
        let wins = self.stats.wins;
        self.stats.rates = PlayerRates {
            win_rate: ratio(wins, hands),
            tsumo_win_rate: ratio(self.stats.tsumo_wins, hands),
            ron_win_rate: ratio(self.stats.ron_wins, hands),
            tsumo_share: ratio(self.stats.tsumo_wins, wins),
            ron_share: ratio(self.stats.ron_wins, wins),
            deal_in_rate: ratio(self.stats.deal_ins, hands),
            riichi_rate: ratio(self.stats.riichi, hands),
            call_rate: ratio(self.stats.called_hands, hands),
            dealer_repeat_rate: ratio(self.stats.dealer_repeats, self.stats.dealer_hands),
            tenpai_rate: ratio(self.stats.tenpai_draws, self.stats.exhaustive_draws),
        };
        self.stats.bonuses.dora_hit_rate = ratio(self.stats.bonuses.wins_with_dora, wins);
        self.stats.bonuses.ura_dora_hit_rate = ratio(self.stats.bonuses.wins_with_ura_dora, wins);
        self.stats.bonuses.aka_dora_hit_rate = ratio(self.stats.bonuses.wins_with_aka_dora, wins);
        self.stats.yaku_frequency = self
            .yaku
            .into_iter()
            .map(|(id, (name, count))| YakuCount { id, name, count })
            .collect();
        self.stats.han_distribution = into_buckets(self.han);
        self.stats.fu_distribution = into_buckets(self.fu);
        self.stats.hand_value_distribution = into_buckets(self.hand_value);
        self.stats
    }
}

pub fn summarize_game(log_id: String, parsed: ParsedGame) -> GameDetail {
    let ParsedGame {
        rules,
        players,
        kyoku,
        final_scores,
        uma,
    } = parsed;
    let active_seats = players
        .iter()
        .filter(|player| !player.name.is_empty())
        .map(|player| player.seat)
        .collect::<Vec<_>>();
    let mut accumulators = (0..4)
        .map(|_| StatsAccumulator::default())
        .collect::<Vec<_>>();
    let mut deal_ins = vec![vec![0; 4]; 4];

    for (index, hand) in kyoku.iter().enumerate() {
        for seat in &active_seats {
            accumulators[usize::from(*seat)].stats.hands += 1;
        }
        if active_seats.contains(&hand.dealer) {
            let dealer = &mut accumulators[usize::from(hand.dealer)].stats;
            dealer.dealer_hands += 1;
            if kyoku
                .get(index + 1)
                .is_some_and(|next| next.round_index == hand.round_index)
            {
                dealer.dealer_repeats += 1;
            }
        }

        let mut called = [false; 4];
        for event in &hand.events {
            match event {
                TurnEvent::Riichi { seat } => accumulators[usize::from(*seat)].stats.riichi += 1,
                TurnEvent::Call { seat, kind, .. } => {
                    called[usize::from(*seat)] = true;
                    accumulators[usize::from(*seat)].record_call(*kind);
                }
                _ => {}
            }
        }
        for seat in &active_seats {
            accumulators[usize::from(*seat)].stats.called_hands +=
                u32::from(called[usize::from(*seat)]);
        }

        match &hand.result {
            KyokuResult::Win { wins } => {
                for win in wins {
                    accumulators[usize::from(win.winner)].record_win(win);
                    if !win.tsumo {
                        accumulators[usize::from(win.from_seat)].stats.deal_ins += 1;
                        deal_ins[usize::from(win.from_seat)][usize::from(win.winner)] += 1;
                    }
                }
            }
            KyokuResult::Draw {
                reason: DrawReason::Exhaustive,
                tenpai_seats,
                ..
            } => {
                for seat in &active_seats {
                    let stats = &mut accumulators[usize::from(*seat)].stats;
                    stats.exhaustive_draws += 1;
                    if tenpai_seats.contains(seat) {
                        stats.tenpai_draws += 1;
                    } else {
                        stats.noten_draws += 1;
                    }
                }
            }
            KyokuResult::Draw { .. } => {}
        }
    }

    let fallback_scores = kyoku
        .last()
        .map(|hand| hand.end_scores.clone())
        .unwrap_or_default();
    let final_scores = final_scores.unwrap_or(fallback_scores);
    let placements = placements(&active_seats, &final_scores);
    let mut summaries = Vec::with_capacity(active_seats.len());
    for player in players.into_iter().filter(|player| !player.name.is_empty()) {
        let seat = usize::from(player.seat);
        summaries.push(PlayerSummary {
            seat: player.seat,
            name: player.name,
            rank_id: player.rank_id,
            rank: player.rank,
            rating: player.rating,
            sex: player.sex,
            final_score: final_scores.get(seat).copied(),
            uma: uma.as_ref().and_then(|values| values.get(seat)).copied(),
            placement: placements[seat],
            stats: std::mem::take(&mut accumulators[seat]).finish(),
        });
    }

    let matrix_counts = summaries
        .iter()
        .map(|from| {
            summaries
                .iter()
                .map(|to| deal_ins[usize::from(from.seat)][usize::from(to.seat)])
                .collect()
        })
        .collect();
    let matrix_players = summaries.iter().map(|player| player.name.clone()).collect();

    GameDetail {
        log_id,
        rules,
        players: summaries,
        kyoku,
        deal_in_matrix: DealInMatrix {
            players: matrix_players,
            counts: matrix_counts,
        },
    }
}

pub fn aggregate_career(games: &[GameDetail], player_names: &[String]) -> CareerStats {
    let selected = player_names
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let relevant = games
        .iter()
        .filter(|game| {
            game.players
                .iter()
                .any(|player| selected.contains(player.name.as_str()))
        })
        .collect::<Vec<_>>();
    let mut accumulator = StatsAccumulator::default();
    let mut placements = BTreeMap::new();
    let mut placement_total = 0_u32;
    let mut placement_count = 0_u32;
    let mut score_trend = Vec::new();

    for game in &relevant {
        for player in game
            .players
            .iter()
            .filter(|player| selected.contains(player.name.as_str()))
        {
            accumulator.merge(&player.stats);
            if let Some(placement) = player.placement {
                increment(&mut placements, u32::from(placement), 1);
                placement_total += u32::from(placement);
                placement_count += 1;
            }
            if let (Some(final_score), Some(placement)) = (player.final_score, player.placement) {
                score_trend.push(ScoreTrendPoint {
                    log_id: game.log_id.clone(),
                    player_name: player.name.clone(),
                    final_score,
                    placement,
                });
            }
        }
    }

    let matrix_players = relevant
        .iter()
        .flat_map(|game| game.deal_in_matrix.players.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let matrix_index = matrix_players
        .iter()
        .enumerate()
        .map(|(index, name)| (name.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let mut matrix_counts = vec![vec![0; matrix_players.len()]; matrix_players.len()];
    for game in relevant.iter() {
        for (from_index, from_name) in game.deal_in_matrix.players.iter().enumerate() {
            for (to_index, to_name) in game.deal_in_matrix.players.iter().enumerate() {
                matrix_counts[matrix_index[from_name.as_str()]][matrix_index[to_name.as_str()]] +=
                    game.deal_in_matrix.counts[from_index][to_index];
            }
        }
    }

    CareerStats {
        player_names: player_names.to_vec(),
        games: relevant.len() as u32,
        average_placement: (placement_count > 0).then(|| f64::from(placement_total) / f64::from(placement_count)),
        stats: accumulator.finish(),
        placement_distribution: into_buckets(placements),
        score_trend,
        deal_in_matrix: DealInMatrix {
            players: matrix_players,
            counts: matrix_counts,
        },
    }
}

fn placements(active_seats: &[u8], scores: &[i32]) -> [Option<u8>; 4] {
    let mut ranked = active_seats
        .iter()
        .map(|seat| {
            (
                *seat,
                scores.get(usize::from(*seat)).copied().unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|(left_seat, left_score), (right_seat, right_score)| {
        right_score
            .cmp(left_score)
            .then_with(|| left_seat.cmp(right_seat))
    });
    let mut result = [None; 4];
    for (index, (seat, _)) in ranked.into_iter().enumerate() {
        result[usize::from(seat)] = Some(index as u8 + 1);
    }
    result
}

fn ratio(numerator: u32, denominator: u32) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        f64::from(numerator) / f64::from(denominator)
    }
}

fn increment(map: &mut BTreeMap<u32, u32>, key: u32, amount: u32) {
    *map.entry(key).or_default() += amount;
}

fn into_buckets(map: BTreeMap<u32, u32>) -> Vec<CountBucket> {
    map.into_iter()
        .map(|(value, count)| CountBucket { value, count })
        .collect()
}

fn merge_buckets(target: &mut BTreeMap<u32, u32>, source: &[CountBucket]) {
    for bucket in source {
        increment(target, bucket.value, bucket.count);
    }
}

fn merge_calls(target: &mut CallStats, source: &CallStats) {
    target.total += source.total;
    target.chi += source.chi;
    target.pon += source.pon;
    target.open_kan += source.open_kan;
    target.closed_kan += source.closed_kan;
    target.added_kan += source.added_kan;
    target.nuki += source.nuki;
}

fn merge_bonuses(target: &mut BonusStats, source: &BonusStats) {
    target.dora += source.dora;
    target.ura_dora += source.ura_dora;
    target.aka_dora += source.aka_dora;
    target.wins_with_dora += source.wins_with_dora;
    target.wins_with_ura_dora += source.wins_with_ura_dora;
    target.wins_with_aka_dora += source.wins_with_aka_dora;
}
