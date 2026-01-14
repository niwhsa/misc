fn merge_sort<T: Ord + Copy>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);
    merge(&left, &right)
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

fn main() {
    let arr = vec![64, 34, 25, 12, 22, 11, 90];
    let sorted = merge_sort(&arr);

    assert_eq!(sorted, vec![11, 12, 22, 25, 34, 64, 90]);
    println!("Sorted: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(merge_sort(&arr), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(merge_sort(&arr), vec![]);
    }

    #[test]
    fn test_merge_sort_single() {
        let arr = vec![42];
        assert_eq!(merge_sort(&arr), vec![42]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reversed() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_strings() {
        let arr = vec!["banana", "apple", "cherry"];
        assert_eq!(merge_sort(&arr), vec!["apple", "banana", "cherry"]);
    }
}
