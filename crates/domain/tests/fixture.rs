use domain::{
    parse_game,
    stats::aggregate_career,
    types::{CountBucket, KyokuResult, PlayerSummary, TurnEvent},
};

const LOG_ID: &str = "2017010100gm-00a9-0000-003dbd5d";
const XML: &str = include_str!("fixtures/2017010100gm-00a9-0000-003dbd5d.xml");

fn fixture() -> domain::GameDetail {
    parse_game(LOG_ID, XML).expect("real fixture should parse")
}

fn player(detail: &domain::GameDetail, seat: u8) -> &PlayerSummary {
    detail
        .players
        .iter()
        .find(|player| player.seat == seat)
        .expect("fixture has all four seats")
}

fn bucket(distribution: &[CountBucket], value: u32) -> u32 {
    distribution
        .iter()
        .find(|bucket| bucket.value == value)
        .map_or(0, |bucket| bucket.count)
}

#[test]
fn parses_real_game_metadata_rounds_and_ordered_events() {
    let detail = fixture();

    assert_eq!(detail.log_id, LOG_ID);
    assert_eq!(detail.rules.raw_type, 169);
    assert_eq!(detail.rules.table, "tenhou");
    assert!(detail.rules.hanchan);
    assert!(detail.rules.aka_dora);
    assert!(detail.rules.kuitan);
    assert!(!detail.rules.sanma);
    assert_eq!(detail.players[0].name, "A\u{77f3}\u{6a4b}");
    assert_eq!(detail.players[1].name, "CLS");
    assert_eq!(detail.players[1].rank, "8dan");
    assert_eq!(detail.kyoku.len(), 12);
    assert_eq!(
        detail
            .kyoku
            .iter()
            .map(|hand| (hand.round_index, hand.honba, hand.dealer))
            .collect::<Vec<_>>(),
        [
            (0, 0, 0),
            (1, 0, 1),
            (1, 1, 1),
            (2, 0, 2),
            (2, 1, 2),
            (3, 0, 3),
            (4, 0, 0),
            (5, 0, 1),
            (5, 1, 1),
            (6, 0, 2),
            (6, 1, 2),
            (7, 0, 3),
        ]
    );

    let first = &detail.kyoku[0];
    assert_eq!(first.start_scores, [25_000; 4]);
    assert_eq!(first.end_scores, [25_000, 33_000, 17_000, 25_000]);
    assert_eq!(first.dora_indicators, [66]);
    assert_eq!(first.events[0], TurnEvent::Draw { seat: 0, tile: 49 });
    assert_eq!(
        first.events[1],
        TurnEvent::Discard {
            seat: 0,
            tile: 37,
            tsumogiri: false,
        }
    );
    assert!(first.events.iter().any(|event| matches!(
        event,
        TurnEvent::Discard {
            tile: 38,
            tsumogiri: true,
            ..
        }
    )));
    assert!(first.events.contains(&TurnEvent::Riichi { seat: 1 }));

    let KyokuResult::Win { wins } = &first.result else {
        panic!("first kyoku should be a win");
    };
    assert_eq!(wins.len(), 1);
    assert_eq!((wins[0].winner, wins[0].from_seat), (1, 2));
    assert!(!wins[0].tsumo);
    assert_eq!((wins[0].han, wins[0].fu, wins[0].points), (5, 30, 8_000));

    let exhaustive_draws = detail
        .kyoku
        .iter()
        .filter(|hand| matches!(hand.result, KyokuResult::Draw { .. }))
        .count();
    assert_eq!(exhaustive_draws, 2);
    assert_eq!(
        detail.kyoku[11].end_scores,
        [14_300, 22_900, 19_300, 42_500]
    );
}

#[test]
fn computes_every_required_game_stat_from_the_fixture() {
    let detail = fixture();
    let seat0 = player(&detail, 0);
    let seat1 = player(&detail, 1);
    let seat2 = player(&detail, 2);
    let seat3 = player(&detail, 3);

    assert_eq!(seat0.final_score, Some(14_300));
    assert_eq!(seat1.final_score, Some(22_900));
    assert_eq!(seat2.final_score, Some(19_300));
    assert_eq!(seat3.final_score, Some(43_500));
    assert_eq!(
        detail
            .players
            .iter()
            .map(|player| player.placement)
            .collect::<Vec<_>>(),
        [Some(4), Some(2), Some(3), Some(1)]
    );

    assert_eq!((seat1.stats.hands, seat1.stats.wins), (12, 3));
    assert_eq!((seat1.stats.tsumo_wins, seat1.stats.ron_wins), (0, 3));
    assert_eq!((seat2.stats.tsumo_wins, seat2.stats.ron_wins), (2, 1));
    assert_eq!(seat3.stats.wins, 4);
    assert_eq!(
        detail
            .players
            .iter()
            .map(|player| player.stats.deal_ins)
            .collect::<Vec<_>>(),
        [2, 2, 3, 0]
    );
    assert_eq!(detail.deal_in_matrix.counts[0], [0, 0, 1, 1]);
    assert_eq!(detail.deal_in_matrix.counts[1], [0, 0, 0, 2]);
    assert_eq!(detail.deal_in_matrix.counts[2], [0, 3, 0, 0]);

    assert_eq!(
        detail
            .players
            .iter()
            .map(|player| player.stats.riichi)
            .collect::<Vec<_>>(),
        [1, 3, 6, 1]
    );
    assert_eq!(
        (seat1.stats.dealer_hands, seat1.stats.dealer_repeats),
        (4, 2)
    );
    assert_eq!(
        (seat2.stats.dealer_hands, seat2.stats.dealer_repeats),
        (4, 2)
    );
    assert_eq!(seat1.stats.rates.dealer_repeat_rate, 0.5);

    assert_eq!(seat0.stats.calls.total, 3);
    assert_eq!((seat0.stats.calls.chi, seat0.stats.calls.pon), (1, 2));
    assert_eq!((seat1.stats.calls.total, seat1.stats.calls.chi), (2, 2));
    assert_eq!((seat2.stats.calls.total, seat2.stats.calls.pon), (1, 1));
    assert_eq!(seat3.stats.calls.total, 3);
    assert_eq!((seat3.stats.calls.chi, seat3.stats.calls.pon), (2, 1));
    assert_eq!(seat0.stats.called_hands, 2);

    assert_eq!(
        (seat1.stats.exhaustive_draws, seat1.stats.tenpai_draws),
        (2, 1)
    );
    assert_eq!((seat3.stats.tenpai_draws, seat3.stats.noten_draws), (0, 2));
    assert_eq!(seat1.stats.rates.tenpai_rate, 0.5);

    assert_eq!(seat1.stats.bonuses.dora, 2);
    assert_eq!(seat1.stats.bonuses.ura_dora, 1);
    assert_eq!(seat1.stats.bonuses.aka_dora, 2);
    assert_eq!(seat1.stats.bonuses.wins_with_dora, 2);
    assert_eq!(seat1.stats.bonuses.wins_with_ura_dora, 1);
    assert_eq!(seat2.stats.bonuses.ura_dora, 0);
    assert_eq!(seat1.stats.bonuses.dora_hit_rate, 2.0 / 3.0);

    assert_eq!(
        seat1
            .stats
            .yaku_frequency
            .iter()
            .find(|yaku| yaku.name == "Ura Dora")
            .map(|yaku| yaku.count),
        Some(1)
    );
    assert_eq!(bucket(&seat1.stats.han_distribution, 2), 1);
    assert_eq!(bucket(&seat1.stats.han_distribution, 4), 1);
    assert_eq!(bucket(&seat1.stats.han_distribution, 5), 1);
    assert_eq!(bucket(&seat1.stats.fu_distribution, 30), 3);
    assert_eq!(bucket(&seat1.stats.hand_value_distribution, 2_900), 1);
    assert_eq!(bucket(&seat1.stats.hand_value_distribution, 7_700), 1);
    assert_eq!(bucket(&seat1.stats.hand_value_distribution, 8_000), 1);

    assert_eq!(seat1.stats.rates.win_rate, 0.25);
    assert_eq!(seat2.stats.rates.tsumo_share, 2.0 / 3.0);
    assert_eq!(seat0.stats.rates.deal_in_rate, 1.0 / 6.0);
    assert_eq!(seat0.stats.rates.call_rate, 1.0 / 6.0);
}

#[test]
fn aggregates_career_stats_and_serializes_the_domain_contract() {
    let detail = fixture();
    let career = aggregate_career(std::slice::from_ref(&detail), &["CLS".to_owned()]);

    assert_eq!(career.games, 1);
    assert_eq!(career.average_placement, Some(2.0));
    assert_eq!(career.stats.hands, 12);
    assert_eq!(career.stats.wins, 3);
    assert_eq!(
        career.placement_distribution,
        [CountBucket { value: 2, count: 1 }]
    );
    assert_eq!(career.score_trend.len(), 1);
    assert_eq!(career.score_trend[0].final_score, 22_900);
    assert_eq!(career.deal_in_matrix.players.len(), 4);

    let json = serde_json::to_value(detail).expect("GameDetail should serialize");
    assert_eq!(json["logId"], LOG_ID);
    assert_eq!(json["kyoku"][0]["events"][0]["type"], "Draw");
    assert_eq!(json["players"][1]["stats"]["wins"], 3);
}

#[test]
fn frontend_replay_fixture_matches_the_domain_serialization() {
    let exported: serde_json::Value = serde_json::from_str(include_str!(
        "../../../apps/web/tests/fixtures/2017010100gm-00a9-0000-003dbd5d.json"
    ))
    .expect("frontend replay fixture should be valid JSON");
    let parsed = serde_json::to_value(fixture()).expect("GameDetail should serialize");

    assert_eq!(exported, parsed);
}
