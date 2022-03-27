#![feature(generic_arg_infer)]

use bitum::bit_pos::BitPosition;
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
fn serialize_test() {
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
