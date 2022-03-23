use crate::bit_pos::BitPosition;
use crate::des::BitumDeserializeOwned;

impl<T, const C: usize> BitumDeserializeOwned for [T; C]
where
    T: BitumDeserializeOwned,
{
    fn deserialize_at<const N: usize>(data: &[u8; N], pos: BitPosition) -> (Self, BitPosition) {
        let mut slice: [T; C] = unsafe { std::mem::zeroed() };
        let mut pos = pos;

        for i in 0..C {
            (slice[i], pos) = T::deserialize_at(data, pos);
        }
        
        (slice, pos)
    }
}
