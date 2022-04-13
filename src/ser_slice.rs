use crate::bit_pos::BitPosition;
use crate::ser::BitumSerializeOwned;

impl<T, const C: usize> BitumSerializeOwned for [T; C]
where
    T: BitumSerializeOwned,
{
    fn serialize_into(&self, buffer: &mut [u8], pos: BitPosition) -> BitPosition {
        let mut pos = pos;

        for i in 0..C {
            pos = self[i].serialize_into(buffer, pos);
        }

        pos
    }
}
