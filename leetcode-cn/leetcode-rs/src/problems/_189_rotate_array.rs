pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let size = nums.len();
        let k_usize = k as usize % size;
        let mut count = 0;
        let mut temp: i32;
        let mut start_idx = 0;
        let mut curr_idx = 0;
        while count < size {
            temp = nums[curr_idx];
            loop {
                curr_idx += k_usize;
                curr_idx %= size;
                Self::swap(&mut nums[curr_idx], &mut temp);
                count += 1;
                if curr_idx == start_idx {
                    break;
                }
            }
            start_idx += 1;
            start_idx %= size;
            curr_idx = start_idx;
        }
    }

    fn swap(left: &mut i32, right: &mut i32) {
        let temp = *left;
        *left = *right;
        *right = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let mut v = vec![-1, -100, 3, 99];
        Solution::rotate(&mut v, 2);
        assert_eq!(v, vec![3, 99, -1, -100]);
    }
}
