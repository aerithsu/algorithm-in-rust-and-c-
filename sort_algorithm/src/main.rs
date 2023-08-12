use rand::Rng;
fn main() {
    test();
}
//选择排序,从头往后排序,第i轮找到第i小的元素,与arr[i]交换位置
fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len - 1 {
        //这里轮数为i-1是因为排完前i-1个,整个数组最后一个元素便有序了
        let mut min = i;
        for j in i + 1..len {
            //j可以从i+1处开始,无需比较a[i]与a[i]的大小
            //内层循环寻找第i小的元素,每一轮要扫描的元素越来越少
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}
fn selection_sort2<T: PartialOrd>(arr: &mut [T]) {}
//选择排序,从后往前排,第i轮找到第i大的值(位于len-1-i下标处)
fn select_sort2<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len - 1 {
        let mut max = 0;
        for j in 0..len - i {
            //内层循环寻找第i大的元素
            if arr[j] > arr[max] {
                max = j;
            }
        }
        //第i大的元素和倒数第i个元素互换位置
        arr.swap(max, len - i - 1);
    }
}
//
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - i - 1 {
            //注意上界为len-i-1,当i=0时j=len-0-1刚好不会越界
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        //这里i也可以为0..len-1,相应的j就是(1..=i+1).rev()
        for j in (1..=i).rev() {
            //从后向前扫描,第0轮拍好了前1个元素,第一轮前2个,第len-2轮前len-1个(整个数组便有序了)
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}
fn test() {
    let mut rng = rand::thread_rng();
    let array_size = 100;
    let mut vec = Vec::new();
    for _ in 0..array_size {
        vec.push(rng.gen::<i32>());
    }
    let mut vec1 = vec![2, 3, 1, 5];
    let mut vec2 = vec![1.2, 2.1, 0.5, -2.3];
    insertion_sort(&mut vec1);
    insertion_sort(&mut vec);
    println!("{:?}", vec1);
    println!("{:?}", vec);
}
