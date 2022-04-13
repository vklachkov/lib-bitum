use std::slice::SliceIndex;

use crate::bit_pos::BitPosition;
use crate::des::*;

impl BitumDeserializeOwned for bool {
    fn deserialize_at(buffer: &[u8], pos: BitPosition) -> (Self, BitPosition) {
        let data = buffer[pos.byte];
        let mask = 1 << pos.bit;
        let value = (data & mask) >> pos.bit;

        let result = value == 1;
        let pos = pos.inc_bits(1);

        (result, pos)
    }
}

impl BitumDeserializeSomeBitsOwned for bool {
    fn deserialize_bits_at(
        buffer: &[u8],
        count: usize,
        pos: BitPosition,
    ) -> (Self, BitPosition) {
        assert!(count == 1);

        Self::deserialize_at(buffer, pos)
    }
}
