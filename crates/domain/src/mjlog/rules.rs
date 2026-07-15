use crate::types::Ruleset;

pub fn decode_rules(raw_type: u16, lobby: String) -> Ruleset {
    let test = raw_type & 0x01 == 0;
    let tokujou = raw_type & 0x20 != 0;
    let joukyu = raw_type & 0x80 != 0;
    let table = if tokujou && joukyu {
        "tenhou"
    } else if test {
        "test"
    } else if tokujou {
        "tokujou"
    } else if joukyu {
        "joukyu"
    } else {
        "dan-i"
    };

    Ruleset {
        raw_type,
        lobby,
        test,
        aka_dora: raw_type & 0x02 == 0,
        kuitan: raw_type & 0x04 == 0,
        hanchan: raw_type & 0x08 != 0,
        sanma: raw_type & 0x10 != 0,
        fast: raw_type & 0x40 != 0,
        table: table.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::decode_rules;

    #[test]
    fn decodes_tenhou_hanchan_flags() {
        let rules = decode_rules(169, "0".to_owned());

        assert!(rules.aka_dora);
        assert!(rules.kuitan);
        assert!(rules.hanchan);
        assert!(!rules.sanma);
        assert!(!rules.fast);
        assert_eq!(rules.table, "tenhou");
    }
}
