pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    quick_sort_portion(arr, 0, arr.len() - 1);
}

fn quick_sort_portion<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot_idx = partition(&mut arr[left..=right]);
    quick_sort_portion(
        arr,
        left,
        if left + pivot_idx > 0 {
            left + pivot_idx - 1
        } else {
            0
        },
    );
    quick_sort_portion(arr, left + pivot_idx + 1, right);
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let (init, last) = arr.split_at_mut(arr.len() - 1);
    let pivot = &last[0];
    let mut temp_pivot = 0;
    for idx in 0..init.len() {
        if init[idx] <= *pivot {
            init.swap(idx, temp_pivot);
            temp_pivot += 1;
        }
    }
    arr.swap(temp_pivot, arr.len() - 1);
    temp_pivot
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![] as Vec<i32>);
    }

    #[test]
    fn test_one() {
        let mut arr = vec![3];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![3]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_pivot_with_duplicate() {
        let mut arr = vec![3, 1, 1, 3, 7, 6, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 3, 3, 3, 6, 7]);
    }

    #[test]
    fn test_pivot_no_move_should_not_stack_overflow() {
        let mut arr = vec![3, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![3, 5]);
    }

    #[test]
    fn test_1() {
        let mut arr = vec![3, 5, 7, 8, 4, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![2, 3, 4, 5, 7, 8]);
    }
}
