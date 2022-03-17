mod bitum;

use crate::bitum::*;

fn main() {
    let data: [u8; 16] = [0xF4, 0xA5, 0xB9, 0xDD, 0xF4, 0xA5, 0xB9, 0xDD, 0xF4, 0xA5, 0xB9, 0xDD, 0xF4, 0xA5, 0xB9, 0xDD];
    let number: [u8; 16] = [0xF4, 0xA5, 0xB9, 0xDD, 0xF4, 0xA5, 0xB9, 0xDD, 0xF4, 0xA5, 0xB9, 0xDD, 0xF4, 0xA5, 0xB9, 0xDD];

    let number = u128::from_le_bytes(number);
    println!("Number {number:#X} ({number:#08b})");

    for byte in 0..1 {
        for bit in 0..8 {
            let position = BitPosition::new(byte, bit);

            println!("Position {byte}.{bit}");

            let result = u64::extract(&data, &position);
            println!("Extract: {result:#X} ({result:16b})");

            let v = (number >> position.bits()) as u64;
            println!("Shift: {v:#X} ({v:16b})");

            assert!(result == v);

            println!("------------------------------")
        }
    }
}