use std::io::{self, Write};

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    
    for i in 0..n {
        let mut min_index = i;

        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

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
    let mut input = String::new();

    print!("Enter the numbers separated by spaces: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read the line.");

    let mut arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Enter valid numbers"))
        .collect();

    println!("Unsorted array: {:?}", arr);

    selection_sort(&mut arr);

    println!("Sorted array: {:?}", arr);

    input.clear();
    print!("Enter the target number to search: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read the line.");
    let target: i32 = input.trim().parse().expect("Enter a valid number.");

    match binary_search(&arr, target) {
        Some(index) => println!("Found number {} at index {}!", target, index),
        None => println!("Target number {} was not found in the array.", target),
    }
}
