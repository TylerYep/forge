use std::cmp::Ordering;

use rand::Rng;

pub fn quick_select<T: Ord + Copy>(items: Vec<T>, index: usize) -> T {
    let pivot = items[rand::thread_rng().gen_range(0..items.len())];
    let mut equal = 0;
    let mut smaller = vec![];
    let mut larger = vec![];
    for item in items {
        match item.cmp(&pivot) {
            Ordering::Equal => equal += 1,
            Ordering::Greater => larger.push(item),
            Ordering::Less => smaller.push(item),
        }
    }
    let m = smaller.len();
    match index.cmp(&m) {
        Ordering::Equal => pivot,
        Ordering::Greater => quick_select(larger, index - (m + equal)),
        Ordering::Less => quick_select(smaller, index),
    }
}
