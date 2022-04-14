use crate::bit_pos::BitPosition;
use crate::ser::BitumSerializeOwned;

impl<T> BitumSerializeOwned for Option<T>
where
    T: BitumSerializeOwned,
{
    fn serialize_into(&self, buffer: &mut [u8], pos: BitPosition) -> BitPosition {
        match self {
            Some(v) => v.serialize_into(buffer, pos),
            None => pos,
        }
    }
}
