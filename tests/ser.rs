#![feature(generic_arg_infer)]

use bitum::bit_pos::BitPosition;
use bitum::bits::Bits;
use bitum::ser::BitumSerializeOwned;
use bitum::des::BitumDeserializeOwned;
use bitum_derive::{BitumSerialize, BitumDeserialize};

#[derive(BitumSerialize, BitumDeserialize, PartialEq, Eq, Debug)]
struct SerializeTestStruct {
    f1: u8,
    f2: u16,
    f3: u32, 
    f4: u64,
    f5: u128,
    f6: bool,
}

#[test]
fn serialize() {
    let test_struct = SerializeTestStruct {
        f1: 237,
        f2: 14683,
        f3: 39494755,
        f4: 23453475812,
        f5: 92357239581257,
        f6: true,
    };

    let mut data = [0u8; 128];

    test_struct.serialize(&mut data);

    println!("{:?}", &data);

    let test_struct_deserialized = SerializeTestStruct::deserialize(&data).0;

    assert_eq!(test_struct, test_struct_deserialized);
}

#[derive(BitumSerialize, BitumDeserialize, PartialEq, Eq, Debug)]
struct SerializeBitsStruct {
    f1: u8,
    f2: Bits<u8, 4>,
    f3: Bits<u16, 12>,
    f4: Bits<u32, 27>,
    f5: u32,
    f6: Bits<u64, 49>,
    f7: Bits<u128, 103>,
    f8: u128,
}

#[test]
fn serialize_bits() {
    let test_struct = SerializeBitsStruct {
        f1: 28,
        f2: Bits::new(11),
        f3: Bits::new(1976),
        f4: Bits::new(97415329),
        f5: 2068975841,
        f6: Bits::new(247700840),
        f7: Bits::new(4802612938586005925689756456047),
        f8: 95439399058572600476228442793117852814,
    };

    let mut data = [0u8; 128];

    test_struct.serialize(&mut data);

    let test_struct_deserialized = SerializeBitsStruct::deserialize(&data).0;

    assert_eq!(test_struct, test_struct_deserialized);
}