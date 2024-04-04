fn maxSubarraySum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    let arr = [5,4,-1,7,8];
    let max_sum = maxSubarraySum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
