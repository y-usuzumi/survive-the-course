struct Solution;

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len() as i32;
        let mut result = Vec::with_capacity(nums.len());

        let mut lowest_idx: i32 = 0;
        while lowest_idx < size && nums[lowest_idx as usize] < 0 {
            lowest_idx += 1;
        }

        let mut negative_arr_idx = lowest_idx - 1;
        let mut positive_arr_idx = lowest_idx;

        for num in nums.iter_mut() {
            *num = *num * *num;
        }

        while negative_arr_idx >= 0 && positive_arr_idx < size {
            let (left, right) = (nums[negative_arr_idx as usize], nums[positive_arr_idx as usize]);

            if left < right {
                result.push(left);
                negative_arr_idx -= 1;
            } else {
                result.push(right);
                positive_arr_idx += 1;
            }
        }

        while negative_arr_idx >= 0 {
            result.push(nums[negative_arr_idx as usize]);
            negative_arr_idx -= 1;
        }

        while positive_arr_idx < size {
            result.push(nums[positive_arr_idx as usize]);
            positive_arr_idx += 1;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
        assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), vec![4,9,9,49,121]);
        assert_eq!(Solution::sorted_squares(vec![-1]), vec![1]);
    }
}