use serde::Serialize;

/// Calculate the size of a serialized object.
pub fn size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}

