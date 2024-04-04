fn mergeSortedArrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut mergedArray = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            mergedArray.push(arr1[i]);
            i += 1;
        } else {
            mergedArray.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        mergedArray.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        mergedArray.push(arr2[j]);
        j += 1;
    }

    mergedArray
}

fn main() {
    let arr1 = [1, 3, 5, 57, 99];
    let arr2 = [2, 4, 6, 8, 10];
    let mergedArray = mergeSortedArrays(&arr1, &arr2);
    println!("Merged sorted array: {:?}", mergedArray);
}
