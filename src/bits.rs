use crate::bit_pos::BitPosition;
use crate::des::*;
use crate::ser::*;
use std::fmt::Debug;
use std::ops::Deref;

pub struct Bits<T, const SIZE: usize = 1> {
    value: T,
}

impl<T, const SIZE: usize> Bits<T, SIZE> {
    pub fn new(value: T) -> Self {
        Bits { value }
    }
}

impl<T: Debug, const SIZE: usize> Debug for Bits<T, SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.value))
    }
}

impl<T, const SIZE: usize> Deref for Bits<T, SIZE> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: PartialEq, const SIZE: usize> PartialEq for Bits<T, SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, const SIZE: usize> Eq for Bits<T, SIZE> where T: Eq {}

impl<T: BitumDeserializeSomeBitsOwned, const SIZE: usize> BitumDeserializeOwned for Bits<T, SIZE> {
    fn deserialize_at<const N: usize>(data: &[u8; N], pos: BitPosition) -> (Self, BitPosition) {
        let result = T::deserialize_bits_at(data, SIZE, pos);
        (Bits { value: result.0 }, result.1)
    }
}

impl<T: BitumSerializeSomeBitsOwned, const SIZE: usize> BitumSerializeOwned for Bits<T, SIZE> {
    fn serialize_into<const N: usize>(&self, data: &mut [u8; N], pos: BitPosition) -> BitPosition {
        self.value.serialize_bits_into(data, SIZE, pos)
    }
}
