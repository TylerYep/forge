use std::cmp::Ordering;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let n = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - 1 - i {
            match arr[j].cmp(&arr[j + 1]) {
                Ordering::Equal | Ordering::Less => (),
                Ordering::Greater => arr.swap(j, j + 1),
            }
        }
    }
}
