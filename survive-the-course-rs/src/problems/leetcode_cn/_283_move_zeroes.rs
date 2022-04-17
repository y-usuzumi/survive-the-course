pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut processed = 0;
        for idx in 0..nums.len() {
            let item = nums[idx];
            if item == 0 {
                continue;
            }
            nums[processed] = item;
            processed += 1;
        }
        for idx in processed..nums.len() {
            nums[idx] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);

        assert_eq!(v, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_2() {
        let mut v = vec![0];
        Solution::move_zeroes(&mut v);

        assert_eq!(v, vec![0]);
    }

    #[test]
    fn test_3() {
        let mut v = vec![1];
        Solution::move_zeroes(&mut v);

        assert_eq!(v, vec![1]);
    }
}
