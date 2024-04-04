fn firstOccurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let arr = vec![1, 2, 3, 3, 4, 5];
    let target = 3;
    let output = firstOccurrence(&arr, target);
    println!("First occurrence of {} in {:?} is {:?}", target, arr, output);
}