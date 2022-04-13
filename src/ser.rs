use crate::bit_pos::BitPosition;

pub trait BitumSerializeOwned: Sized {
    fn serialize(&self, buffer: &mut [u8]) -> BitPosition {
        self.serialize_into(buffer, BitPosition::zero())
    }

    fn serialize_into(&self, buffer: &mut [u8], pos: BitPosition) -> BitPosition;
}

pub trait BitumSerializeSomeBitsOwned: Sized {
    fn serialize_bits(&self, buffer: &mut [u8], count: usize) -> BitPosition {
        self.serialize_bits_into(buffer, count, BitPosition::zero())
    }

    fn serialize_bits_into(&self, buffer: &mut [u8], count: usize, pos: BitPosition)
        -> BitPosition;
}
