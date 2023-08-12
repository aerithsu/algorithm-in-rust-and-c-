static N: i32 = 33;
use std::time::SystemTime;
fn main() {
    println!("Hello, world!");
}
fn sum_of_n(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}
fn sum_of_n2(n: i64) -> i64 {
    (n + 1) * n / 2
}
#[test]
fn test1() {
    for _ in 0..5 {
        let now = SystemTime::now();
        sum_of_n2(20000000);
        let duration = now.elapsed().unwrap();
        let time = duration.as_millis();
        println!("func used {time}ms!");
    }
}
//检查两个字符串是否为乱序字符串(乱序字符串是指一个字符串只是另一个字符串的重新排列)
fn anagram_solution2(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let alist: Vec<char> = s1.chars().collect(); //String和&str类的chars()方法返回迭代器
    let mut blist: Vec<char> = s2.chars().collect();
    let mut pos1: usize = 0;

    while pos1 < s1.len() {
        let mut pos2 = 0;
        let mut found = false;
        while pos2 < alist.len() && !found {
            if alist[pos1] == blist[pos2] {
                found = true;
            } else {
                pos2 += 1;
            }
        }
        if found {
            // 某字符存在于 s2 中，将其替换成 ' ' 避免再次比较
            blist[pos2] = ' ';
        } else {
            return false;
        }
        pos1 += 1;
    }
    true
}
#[test]
fn test2() {
    let now = SystemTime::now();
    assert_eq!(true, anagram_solution4("enough", "egounh"));
    assert_eq!(true, anagram_solution4("heart", "earth"));
    assert_eq!(false, anagram_solution4("hello", "hollo"));
    let duration = now.elapsed().unwrap();
    let time = duration.as_millis();
    println!("func used {time}ms!");
}
//先排序再一次比较即可,复杂度取决于排序算法的复杂度
fn anagram_solution3(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut alist: Vec<char> = s1.chars().collect(); //String和&str类的chars()方法返回迭代器
    let mut blist: Vec<char> = s2.chars().collect();
    alist.sort();
    blist.sort();
    for (i, j) in alist.iter().zip(&blist) {
        if *i != *j {
            return false;
        }
    }
    true
}
/*通过分析可知，s1 和 s2 只含 26 个小写字母，所以用两个长度为 26 的列表，统计各个字符出现的
频次就可以了。每遇到一个字符，就增加该字符在列表对应位置的计数*/
fn anagram_solution4(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut alist = [0; 26];
    let mut blist = [0; 26];
    for (i, j) in s1.chars().zip(s2.chars()) {
        alist[i as usize - b'a' as usize] += 1;
        blist[j as usize - b'a' as usize] += 1;
    }
    if alist == blist {
        return true;
    }
    false
}
