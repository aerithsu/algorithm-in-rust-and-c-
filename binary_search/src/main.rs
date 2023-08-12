fn main() {
    println!("Hello, world!");
}
fn binary_search<T: PartialOrd>(key: T, arr: &[T]) -> Option<usize> {
    let mut lo = 0;
    let mut hi = arr.len() - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] < key {
            lo = mid + 1;
        } else if arr[mid] > key {
            hi = mid - 1;
        } else {
            return Some(mid);
        }
    }
    None
}
#[test]
fn binary_search_tes1() {
    let arr = vec![1, 4, 5, 9, 11, 44];
    assert_eq!(binary_search(1, &arr), Some(0));
    assert_eq!(binary_search(22, &arr), None);
    assert_eq!(binary_search(4, &arr), Some(1));
    assert_eq!(binary_search(5, &arr), Some(2));
    assert_eq!(binary_search(9, &arr), Some(3));
}
