use bitum::{bit_pos::BitPosition, ser::*};

#[test]
fn serialize_u8_rounded_position() {
    const COUNT: usize = 16;

    let numbers: [u8; COUNT] = [
        136, 233, 87, 251, 154, 111, 203, 112, 198, 235, 4, 151, 153, 251, 104, 90,
    ];

    for i in 0..COUNT {
        let mut data: [u8; COUNT] = [0; COUNT];

        numbers[i].serialize_into(&mut data, BitPosition::new(i, 0));

        assert!(data[i] == numbers[i])
    }
}

#[test]
fn serialize_u8_fractional_position() {
    const COUNT: usize = 15;

    let numbers: [u8; COUNT] = [
        136, 233, 87, 251, 154, 111, 203, 112, 198, 4, 151, 153, 251, 104, 90,
    ];

    for i in 0..COUNT {
        for b in 1..8 {
            let mut data: [u8; COUNT + 1] = [0; COUNT + 1];

            numbers[i].serialize_into(&mut data, BitPosition::new(i, b));

            let t = (numbers[i] as u16) << b;

            let s: [u8; 2] = data[i..i + 2].try_into().unwrap();
            let r = u16::from_le_bytes(s);

            assert_eq!(t, r)
        }
    }
}
