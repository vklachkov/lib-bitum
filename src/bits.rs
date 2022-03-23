use crate::bit_pos::BitPosition;
use crate::des::*;
use std::fmt::Debug;
use std::ops::Deref;

pub struct Bits<T, const SIZE: usize = 1>
where
    T: BitumDeserializeSomeBitsOwned + Debug,
{
    value: T,
}

impl<T, const SIZE: usize> Bits<T, SIZE> where T: BitumDeserializeSomeBitsOwned + Debug {}

impl<T, const SIZE: usize> Debug for Bits<T, SIZE>
where
    T: BitumDeserializeSomeBitsOwned + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.value))
    }
}

impl<T, const SIZE: usize> Deref for Bits<T, SIZE>
where
    T: BitumDeserializeSomeBitsOwned + Debug,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, const SIZE: usize> BitumDeserializeOwned for Bits<T, SIZE>
where
    T: BitumDeserializeSomeBitsOwned + Debug,
{
    fn deserialize_at<const N: usize>(data: &[u8; N], pos: BitPosition) -> (Self, BitPosition) {
        let result = T::deserialize_bits_at(data, SIZE, pos);
        (Bits { value: result.0 }, result.1)
    }
}
