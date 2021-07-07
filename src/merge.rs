#![allow(non_snake_case, unused)]
use super::Sorter;
use crate::Ordering;
pub struct Merge;

impl<T: Ordering + Default> Sorter<T> for Merge {
    fn sort(array: &mut [T]) {
        let len = array.len();
        let l = 0_usize;
        let r = len - 1;
        Merge::merge_sort(array, l, r);
    }
}

impl Merge {
    fn merge_sort<T: Ordering + Default>(array: &mut [T], l: usize, r: usize) {
        if l < r {
            let mut m = l + (r - l) / 2;
            Self::merge_sort(array, l, m);
            Self::merge_sort(array, m + 1, r);
            Self::merge(array, l, m, r);
        }
    }

    fn merge<T: Ordering + Default>(array: &mut [T], p: usize, q: usize, r: usize) {
        let n1 = q - p + 1;
        let n2 = r - q;

        // create temporary arrays, left and right
        let mut left: Vec<T> = vec![Default::default(); n1];
        let mut right: Vec<T> = vec![Default::default(); n2];

        // copy array data into left and right arrays
        for i in 0..n1 {
            left[i] = array[p + i];
        }
        for i in 0..n2 {
            right[i] = array[q + 1 + i];
        }

        // Merge temporary arrays back into array[0..r]
        // i is initial index of first subarray
        // j is initial index of second subarray
        // k is initial index of merged subarray

        let (mut i, mut j, mut k) = (0_usize, 0_usize, p);

        while i < n1 && j < n2 {
            if left[i] <= right[j] {
                array[k] = left[i];
                i += 1;
            } else {
                array[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        // Copy the remaining elements of left[] and right[] if left
        while i < n1 {
            array[k] = left[i];
            i += 1;
            k += 1
        }
        while j < n2 {
            array[k] = right[j];
            j += 1;
            k += 1;
        }
    }
}
