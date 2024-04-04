fn medianOfArray(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 9];
    let output = medianOfArray(&arr);
    println!("Median of {:?} is {}", arr, output);
}