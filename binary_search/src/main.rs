fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = arr[mid];

        if guess == target {
            return Some(mid);
        } else if guess > target {
            if mid == 0 { break; }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 5, 7, 9, 12, 32, 43, 55, 67, 98];
    let target = 67;

    match binary_search(&arr, target) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
