#![feature(generic_arg_infer)]

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

            println!("Byte {i} bit {b}");
            assert_eq!(t, r)
        }
    }
}

#[test]
fn serialize_u16_rounded_position() {
    const COUNT: usize = 15;

    let numbers: [u16; COUNT] = [
        11639, 475, 399, 8989, 7803, 6379, 14626, 2343, 5428, 12978, 14231, 16056, 3586, 1772,
        12209,
    ];

    for i in 0..COUNT {
        let mut data: [u8; COUNT + 1] = [0; COUNT + 1];

        numbers[i].serialize_into(&mut data, BitPosition::new(i, 0));

        let s: [u8; 2] = data[i..i + 2].try_into().unwrap();
        let r = u16::from_le_bytes(s);

        assert!(r == numbers[i])
    }
}

#[test]
fn serialize_u16_fractional_position() {
    const COUNT: usize = 14;
    const BUF_SIZE: usize = 64;

    let numbers: [u16; COUNT] = [
        11639, 475, 399, 8989, 7803, 6379, 14626, 2343, 5428, 12978, 14231, 3586, 1772, 12209,
    ];

    for i in 0..COUNT {
        for b in 1..8 {
            let mut data: [u8; BUF_SIZE] = [0; BUF_SIZE];

            numbers[i].serialize_into(&mut data, BitPosition::new(i, b));

            let t = (numbers[i] as u32) << b;

            let s: [u8; 4] = data[i..i + 4].try_into().unwrap();
            let r = u32::from_le_bytes(s);

            println!("Byte {i} bit {b}");
            assert_eq!(t, r)
        }
    }
}

#[test]
fn serialize_u32_rounded_position() {
    const COUNT: usize = 16;
    const BUF_SIZE: usize = 64;

    let numbers: [u32; COUNT] = [
        1399003, 4194645, 4064772, 1677151, 172732, 1820471, 2381941, 961170, 3611365, 417434,
        3375432, 4140831, 2351733, 1809001, 1320285, 382601,
    ];

    for i in 0..COUNT {
        let mut data: [u8; BUF_SIZE] = [0; BUF_SIZE];

        numbers[i].serialize_into(&mut data, BitPosition::new(i, 0));

        let s: [u8; 4] = data[i..i + 4].try_into().unwrap();
        let r = u32::from_le_bytes(s);

        assert!(r == numbers[i])
    }
}

#[test]
fn serialize_u32_fractional_position() {
    const COUNT: usize = 16;
    const BUF_SIZE: usize = 64;

    let numbers: [u32; COUNT] = [
        1399003, 4194645, 4064772, 1677151, 172732, 1820471, 2381941, 961170, 3611365, 417434,
        3375432, 4140831, 2351733, 1809001, 1320285, 382601,
    ];

    for i in 0..COUNT {
        for b in 1..8 {
            let mut data: [u8; BUF_SIZE] = [0; BUF_SIZE];

            numbers[i].serialize_into(&mut data, BitPosition::new(i, b));

            let t = (numbers[i] as u64) << b;

            let s: [u8; 8] = data[i..i + 8].try_into().unwrap();
            let r = u64::from_le_bytes(s);

            println!("Byte {i} bit {b}");
            assert_eq!(t, r)
        }
    }
}

#[test]
fn serialize_slice_dummy() {
    let mut data = [0u8; 64];
    let slice = [12000u16; 16];

    slice.serialize(&mut data);

    println!("{}", 46 << 8 | 224);
    println!("{:?}", &data);
}