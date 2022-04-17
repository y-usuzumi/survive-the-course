use test_util::unconfident;

pub struct Solution;

#[unconfident]
impl Solution {
    pub fn balanced_split_exists(arr: &mut Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum & 1 == 1 {
            return false;
        }

        let sum_target = sum / 2;
        let mut curr_sum = 0;
        let (mut left, mut right) = (0, arr.len() - 1);
        while left < right {
            let (sum, new_pivot_idx) = Self::split_by_pivot_inplace(arr, left, right);
            if curr_sum + sum == sum_target {
                if arr[new_pivot_idx] == 0 {
                    return true;
                }
                return !arr.iter().skip(new_pivot_idx + 1).any(|val| *val == arr[new_pivot_idx]);
            } else if curr_sum + sum > sum_target {
                right = new_pivot_idx;
            } else {
                curr_sum += sum;
                left = new_pivot_idx + 1;
            }
        }

        return curr_sum == sum_target;

    }

    fn split_by_pivot_inplace(arr: &mut Vec<i32>, mut left: usize, mut right: usize) -> (i32, usize) {
        if left >= right {
            return (0, left);
        }
        if right - left == 1 {
            return (arr[left], left);
        }
        let pivot = arr[left];
        let mut left_idx = left + 1;
        let mut right_idx = right - 1;
        let mut sum = pivot;
        while left_idx < right_idx {
            if arr[left_idx] < pivot {
                sum += arr[left_idx];
                left_idx += 1;
                continue;
            }
            while arr[right_idx] >= pivot && right_idx >= left_idx {
                right_idx -= 1;
            }
            if right_idx > left_idx {
                sum += arr[right_idx];
                arr.swap(left_idx, right_idx);
                left_idx += 1;
                right_idx -= 1;
            }
        }

        if left_idx > right_idx {
            arr.swap(right_idx, left);
            return (sum, right_idx);
        } else if arr[left_idx] < pivot {
            sum += arr[left_idx];
            arr.swap(left_idx, left);
            return (sum, left_idx);
        } else {
            arr.swap(left_idx-1, left);
            return (sum, left_idx-1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::balanced_split_exists(&mut vec![2, 1, 2, 5]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::balanced_split_exists(&mut vec![3, 6, 3, 4, 4]), false);
    }
}