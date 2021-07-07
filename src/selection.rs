use crate::Ordering;

use super::Sorter;
pub struct Selection;

impl<T: Ordering> Sorter<T> for Selection {
    fn sort(array: &mut [T]) {

        for i in 0..array.len() {
            let min_index = i + array[i..]
                .iter()
                .enumerate()
                .min_by_key(|(_index, &item)| item)
                .map(|(index, _item)| index)
                .unwrap_or(i);

            array.swap(min_index, i);
        }
    }
}   
