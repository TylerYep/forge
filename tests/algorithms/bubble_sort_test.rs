use forge::algorithms::bubble_sort;
use forge::testutils::is_sorted;

#[test]
fn empty() {
    let mut ve3: Vec<usize> = vec![];
    bubble_sort(&mut ve3);
    assert!(is_sorted(&ve3));
}

#[test]
fn ascending() {
    //pre-sorted
    let mut ve2 = vec![1, 2, 3, 4, 5, 6];
    bubble_sort(&mut ve2);
    assert!(is_sorted(&ve2));
}

#[test]
fn descending() {
    //descending
    let mut ve1 = vec![6, 5, 4, 3, 2, 1];
    bubble_sort(&mut ve1);
    assert!(is_sorted(&ve1));
}
