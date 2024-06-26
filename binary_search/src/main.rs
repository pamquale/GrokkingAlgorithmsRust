fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut top = arr.len() - 1;

    while low <= top {
        let mid = (low + top) / 2;
        let guess = arr[mid];
        let low_num = arr[low];
        let top_num = arr[top];
        println!("low: [{}-{}] mid: [{}-{}] top: [{}-{}] \nguess: {}", low, low_num, mid, guess, top, top_num, guess);

        if guess == target {
            return Some(mid);
        } else if guess > target {
            if mid == 0 { break; }
            top = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];
    let target = 512;

    match binary_search(&arr, target) {
        Some(index) => println!("Found {} at index {}!", target, index),
        None => println!("Item {} was not found.", target),
    }
}