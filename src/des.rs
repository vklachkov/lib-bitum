use crate::bit_pos::BitPosition;

pub trait BitumDeserializeOwned: Sized {
    fn deserialize<const N: usize>(data: &[u8; N]) -> (Self, BitPosition) {
        Self::deserialize_at(data, BitPosition::zero())
    }

    fn deserialize_at<const N: usize>(data: &[u8; N], pos: BitPosition) -> (Self, BitPosition);
}

pub trait BitumDeserializeSomeBitsOwned: Sized {
    fn deserialize_bits<const N: usize>(data: &[u8; N], count: usize) -> (Self, BitPosition) {
        Self::deserialize_bits_at(data, count, BitPosition::zero())
    }

    fn deserialize_bits_at<const N: usize>(
        data: &[u8; N],
        count: usize,
        pos: BitPosition,
    ) -> (Self, BitPosition);
}
