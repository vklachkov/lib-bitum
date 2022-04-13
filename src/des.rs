use crate::bit_pos::BitPosition;

pub trait BitumDeserializeOwned: Sized {
    fn deserialize(buffer: &[u8]) -> (Self, BitPosition) {
        Self::deserialize_at(buffer, BitPosition::zero())
    }

    fn deserialize_at(buffer: &[u8], pos: BitPosition) -> (Self, BitPosition);
}

pub trait BitumDeserializeSomeBitsOwned: Sized {
    fn deserialize_bits(buffer: &[u8], count: usize) -> (Self, BitPosition) {
        Self::deserialize_bits_at(buffer, count, BitPosition::zero())
    }

    fn deserialize_bits_at(
        buffer: &[u8],
        count: usize,
        pos: BitPosition,
    ) -> (Self, BitPosition);
}
