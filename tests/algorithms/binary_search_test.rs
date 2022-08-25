use forge::algorithms::binary_search;

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
