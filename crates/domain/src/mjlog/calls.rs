use crate::types::CallKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedCall {
    pub kind: CallKind,
    pub tiles: Vec<u16>,
    pub from_seat: u8,
}

pub fn decode_call(seat: u8, value: u32) -> DecodedCall {
    let relative = (value & 0x3) as u8;
    let from_seat = (seat + relative) % 4;

    if value & 0x4 != 0 {
        let encoded = (value >> 10) & 0x3f;
        let encoded_type = encoded / 3;
        let tile_type = (encoded_type / 7) * 9 + encoded_type % 7;
        let base = tile_type * 4;
        return DecodedCall {
            kind: CallKind::Chi,
            tiles: vec![
                (base + ((value >> 3) & 0x3)) as u16,
                (base + 4 + ((value >> 5) & 0x3)) as u16,
                (base + 8 + ((value >> 7) & 0x3)) as u16,
            ],
            from_seat,
        };
    }

    if value & 0x8 != 0 {
        let unused = (value >> 5) & 0x3;
        let base = (((value >> 9) & 0x7f) / 3) * 4;
        return DecodedCall {
            kind: CallKind::Pon,
            tiles: (0..4)
                .filter(|copy| *copy != unused)
                .map(|copy| (base + copy) as u16)
                .collect(),
            from_seat,
        };
    }

    if value & 0x10 != 0 {
        let base = (((value >> 9) & 0x7f) / 3) * 4;
        return DecodedCall {
            kind: CallKind::AddedKan,
            tiles: (0..4).map(|copy| (base + copy) as u16).collect(),
            from_seat,
        };
    }

    if value & 0x20 != 0 {
        return DecodedCall {
            kind: CallKind::Nuki,
            tiles: vec![((value >> 8) & 0xff) as u16],
            from_seat,
        };
    }

    let base = (((value >> 8) & 0xff) / 4) * 4;
    DecodedCall {
        kind: if relative == 0 {
            CallKind::ClosedKan
        } else {
            CallKind::OpenKan
        },
        tiles: (0..4).map(|copy| (base + copy) as u16).collect(),
        from_seat,
    }
}

#[cfg(test)]
mod tests {
    use super::decode_call;
    use crate::types::CallKind;

    #[test]
    fn decodes_real_chi_and_pon_values() {
        let chi = decode_call(0, 11_551);
        assert_eq!(chi.kind, CallKind::Chi);
        assert_eq!(chi.tiles, [15, 16, 22]);
        assert_eq!(chi.from_seat, 3);

        let pon = decode_call(0, 30_794);
        assert_eq!(pon.kind, CallKind::Pon);
        assert_eq!(pon.tiles, [80, 81, 83]);
        assert_eq!(pon.from_seat, 2);
    }

    #[test]
    fn classifies_all_call_families() {
        assert_eq!(decode_call(0, 7).kind, CallKind::Chi);
        assert_eq!(decode_call(0, 11).kind, CallKind::Pon);
        assert_eq!(decode_call(0, 19).kind, CallKind::AddedKan);
        assert_eq!(decode_call(0, 35).kind, CallKind::Nuki);
        assert_eq!(decode_call(0, 1).kind, CallKind::OpenKan);
        assert_eq!(decode_call(0, 0).kind, CallKind::ClosedKan);
    }
}
