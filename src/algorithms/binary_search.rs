use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match item.cmp(&arr[mid]) {
            Ordering::Less => high = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => low = mid + 1,
        }
    }
    None
}

// use std::cmp::Ordering;

// pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
//     let mut low = 0;
//     let mut high = arr.len() - 1;
//     let mut mid;
//     while low < high {
//         mid = low + (high - low) / 2;
//         if arr[mid] == *item {
//             return Some(mid);
//         } else if arr[mid] < *item {
//             high = mid + 1;
//         } else if arr[mid] > *item {
//             low = mid - 1;
//         }
//     }
//     None
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn search_ints_asc() {
        let index = binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}