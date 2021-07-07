//! Sorter trait provides only one method `fn sort(slice: &mut [T])`
//! which takes in a mutable reference to your unsorted slice and 
//! sorts it in place. To create a new sorting algorithm, just create
//! a struct and impl Sorter for your struct.
#![feature(trait_alias)]
#![allow(unused)]

mod test;
mod selection;
mod bubble;
mod merge;

pub use selection::Selection as Selection;
pub use bubble::Bubble as Bubble;
pub use merge::Merge as Merge;

/// Shepmaster : if you pass in a f64 slice with NaN in it, how will you sort it?
/// That's an interesting thought. Consider the crate ordered_float if you're
/// feeling too ambitious. For now we stick with Ord and Eq types.

pub trait Ordering = Ord + Copy;
pub trait Sorter<T>
where T: Ord {
    fn sort(slice: &mut [T]);
}


