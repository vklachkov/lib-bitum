use crate::bit_pos::BitPosition;

pub trait BitumSerializeOwned: Sized {
    fn serialize<const N: usize>(&self, data: &mut [u8; N]) -> BitPosition {
        self.serialize_into(data, BitPosition::zero())
    }

    fn serialize_into<const N: usize>(&self, data: &mut [u8; N], pos: BitPosition) -> BitPosition;
}

pub trait BitumSerializeSomeBitsOwned: Sized {
    fn serialize_bits<const N: usize>(&self, data: &mut [u8; N], count: usize) -> BitPosition {
        self.serialize_bits_into(data, count, BitPosition::zero())
    }

    fn serialize_bits_into<const N: usize>(
        &self,
        data: &mut [u8; N],
        count: usize,
        pos: BitPosition,
    ) -> BitPosition;
}
