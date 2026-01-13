fn randomized_quicksort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high { return; }

    let pvt_idx = partition(arr, low, high);
    if pvt_idx > 0 {
        randomized_quicksort(arr, low, pvt_idx - 1);
    }
    randomized_quicksort(arr, pvt_idx + 1, high);
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    use rand::Rng;
    
    // Random pivot selection
    let pivot_idx = rand::thread_rng().gen_range(low..=high);
    arr.swap(pivot_idx, high); // Move pivot to end
    
    let pivot = arr[high];
    let mut i = low;
    
    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, high); // Move pivot to final position
    i
}
fn main() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    let len = arr.len();
    randomized_quicksort(&mut arr, 0, len - 1);
    
    assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    println!("Sorted: {:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let len = arr.len();
        randomized_quicksort(&mut arr, 0, len - 1);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_quicksort_empty() {
        let mut arr: Vec<i32> = vec![];
        // No-op for empty array
        let len = arr.len();
        if len > 0 {
            randomized_quicksort(&mut arr, 0, len - 1);
        }
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_quicksort_single() {
        let mut arr = vec![42];
        randomized_quicksort(&mut arr, 0, 0);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_quicksort_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let len = arr.len();
        randomized_quicksort(&mut arr, 0, len - 1);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_reversed() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let len = arr.len();
        randomized_quicksort(&mut arr, 0, len - 1);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
