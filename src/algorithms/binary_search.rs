use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match item.cmp(&arr[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => low = mid + 1,
            Ordering::Less => high = mid,
        }
    }
    None
}
