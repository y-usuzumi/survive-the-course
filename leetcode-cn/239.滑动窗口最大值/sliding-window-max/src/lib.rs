use std::collections::LinkedList;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return Vec::new();
        }
        let uk = k as usize;
        if uk <= 1 {
            return nums;
        }
        let mut d = LinkedList::new();
        let mut result = Vec::with_capacity(uk);
        for (i, item) in nums.iter().enumerate() {
            while d.len() > 0 && nums[*d.back().unwrap()] < *item {
                d.pop_back();
            }
            d.push_back(i);
            if i >= uk - 1 {
                let idx = *d.front().unwrap();
                result.push(nums[idx]);
                if idx <= i + 1 - uk {
                    d.pop_front();
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let k = 3;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![1,-1];
        let k = 1;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![1, -1]);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![7, 2, 4];
        let k = 2;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![7, 4]);
    }
}
