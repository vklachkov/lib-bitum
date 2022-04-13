use crate::bit_pos::BitPosition;
use crate::ser::*;

impl BitumSerializeOwned for bool {
    fn serialize_into(&self, data: &mut [u8], pos: BitPosition) -> BitPosition {
        let value = if *self { 1 } else { 0 };
        data[pos.byte] |= value << pos.bit;

        pos.inc_bits(1)
    }
}

impl BitumSerializeSomeBitsOwned for bool {
    fn serialize_bits_into(&self, data: &mut [u8], count: usize, pos: BitPosition) -> BitPosition {
        assert!(count == 1);

        let value = if *self { 1 } else { 0 };
        data[pos.byte] |= value << pos.bit;

        pos.inc_bits(1)
    }
}
