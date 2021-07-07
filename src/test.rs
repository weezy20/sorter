#![cfg(test)]
#![allow(unused)]

use super::*;
use rand::prelude::*;

#[test]
fn selection_sort() {
    let mut rng = thread_rng();
    let mut sorted= (1..120).collect::<Vec<u8>>();
    
    let mut unsorted= sorted.clone();
    unsorted.shuffle(&mut rng);

    // println!("Unsorted slice {:?}", unsorted);
    Selection::sort(&mut unsorted);
    assert_eq!(unsorted, sorted);
}

#[test]
fn bubble_sort() {
    let mut rng = thread_rng();
    let mut sorted= (1..120).collect::<Vec<u8>>();
    
    let mut unsorted= sorted.clone();
    unsorted.shuffle(&mut rng);

    // println!("Unsorted slice {:?}", unsorted);
    Bubble::sort(&mut unsorted);
    assert_eq!(unsorted, sorted);
}

