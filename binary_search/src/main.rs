use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = env::args().collect();
    find_not_in_whitelist(&args[1], &args[2]);
}
fn binary_search<T: PartialOrd>(key: T, arr: &[T]) -> Option<usize> {
    let mut lo = 0i32; //当数组中最小元素都大于key时,hi会=-1,不能为usize
    let mut hi: i32 = arr.len() as i32 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid as usize] < key {
            lo = mid + 1;
        } else if arr[mid as usize] > key {
            hi = mid - 1;
        } else {
            return Some(mid as usize);
        }
    }
    None
}
fn read_to_ints(filename: &str) -> Vec<i32> {
    let mut v = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(num) = line.unwrap().parse::<i32>() {
            v.push(num);
        }
    }
    v
}
fn find_not_in_whitelist(filename: &str, whitelist: &str) {
    let numbers = read_to_ints(filename);
    let whitelist = read_to_ints(whitelist);
    println!("{:?}", numbers);
    for num in numbers {
        if binary_search(num, &whitelist) == None {
            println!("{num}"); //不在白名单中的元素打印出来
        }
    }
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
// 1 2 3 find 0
//mid = 1 hi = 0
//mid = 0 hi = -1
