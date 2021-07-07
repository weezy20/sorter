use crate::{Ordering, Sorter};

pub struct Bubble;

impl<T: Ordering> Sorter<T> for Bubble {
    /// Bubble the heavier elements towards the ends
    fn sort(slice: &mut [T]) {
        let len = slice.len();
        for i in 0..len {
            for j in 0..len - 1 {
                if slice[j] > slice[i] {
                    slice.swap(j, i);
                }
            }
        }
    }
}
