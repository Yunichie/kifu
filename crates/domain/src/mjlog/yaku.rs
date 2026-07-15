const YAKU_NAMES: [&str; 55] = [
    "Menzen Tsumo",
    "Riichi",
    "Ippatsu",
    "Chankan",
    "Rinshan",
    "Haitei",
    "Houtei",
    "Pinfu",
    "Tanyao",
    "Iipeiko",
    "Jikaze East",
    "Jikaze South",
    "Jikaze West",
    "Jikaze North",
    "Bakaze East",
    "Bakaze South",
    "Bakaze West",
    "Bakaze North",
    "Yakuhai Haku",
    "Yakuhai Hatsu",
    "Yakuhai Chun",
    "Double Riichi",
    "Chiitoitsu",
    "Chanta",
    "Ittsu",
    "Sanshoku Doujun",
    "Sanshoku Doukou",
    "Sankantsu",
    "Toitoi",
    "Sanankou",
    "Shousangen",
    "Honroutou",
    "Ryanpeikou",
    "Junchan",
    "Honitsu",
    "Chinitsu",
    "Renhou",
    "Tenhou",
    "Chihou",
    "Daisangen",
    "Suuankou",
    "Suuankou Tanki",
    "Tsuuiisou",
    "Ryuuiisou",
    "Chinroutou",
    "Chuurenpoutou",
    "Junsei Chuurenpoutou",
    "Kokushi",
    "Kokushi 13-wait",
    "Daisuushi",
    "Shousuushi",
    "Suukantsu",
    "Dora",
    "Ura Dora",
    "Aka Dora",
];

pub fn yaku_name(id: u8) -> String {
    YAKU_NAMES
        .get(usize::from(id))
        .map(|name| (*name).to_owned())
        .unwrap_or_else(|| format!("Yaku {id}"))
}

#[cfg(test)]
mod tests {
    use super::yaku_name;

    #[test]
    fn maps_bonus_yaku_ids() {
        assert_eq!(yaku_name(52), "Dora");
        assert_eq!(yaku_name(53), "Ura Dora");
        assert_eq!(yaku_name(54), "Aka Dora");
    }
}
