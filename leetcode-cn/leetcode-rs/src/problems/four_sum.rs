struct Solution();

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        Self::sum_in_arr(&nums, target, 4, &mut 0)
    }

    fn sum_in_arr(nums: &[i32], target: i32, remaining_nums: i32, skip_tail: &mut usize) -> Vec<Vec<i32>> {
        if remaining_nums == 0 {
            if target == 0 {
                *skip_tail += nums.len();
                vec![Vec::new()]
            } else {
                vec![]
            }
            
        } else {
            let mut prev_num = std::i32::MIN;
            let mut result = Vec::new();
            let slice = &nums[..nums.len()-*skip_tail];
            for (idx, num) in slice.iter().enumerate() {
                if *num != prev_num {
                    for partial_result in Self::sum_in_arr(&nums[idx+1..nums.len()-*skip_tail], target - num, remaining_nums-1, skip_tail) {
                        result.push(vec![*num].into_iter().chain(partial_result.into_iter()).collect());
                    }
                    prev_num = *num;
                }
            }
            result
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_1() {
        let nums = vec![1,0,-1,0,-2,2];
        let target= 0;
        let actual = Solution::four_sum(nums, target);
        assert_eq!(actual, vec![vec![-2,-1,1,2], vec![-2,0,0,2], vec![-1,0,0,1]]);
    }

    #[test]
    fn test_four_sum_2() {
        let nums = vec![2,2,2,2,2];
        let target= 8;
        let actual = Solution::four_sum(nums, target);
        assert_eq!(actual, vec![vec![2,2,2,2]]);
    }

    #[test]
    fn test_four_sum_3() {
        let nums = vec![-1,0,1,2,-1,-4];
        let target = -1;
        let actual = Solution::four_sum(nums, target);
        assert_eq!(actual, vec![vec![-4,0,1,2],vec![-1,-1,0,1]]);
    }
}
