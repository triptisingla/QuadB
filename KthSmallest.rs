fn kthSmallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }
    let mut sortedArr = arr.to_vec();
    sortedArr.sort();
    Some(sortedArr[k - 1])
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 1;
    match kthSmallest(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid input."),
    }
}
