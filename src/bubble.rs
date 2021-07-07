use crate::{Ordering, Sorter};

pub struct Bubble;

impl<T: Ordering> Sorter<T> for Bubble {
    fn sort(slice: &mut [T]) {
        
    }
}