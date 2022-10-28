// https://leetcode.com/problems/product-of-array-except-self/

pub struct Solution;

impl Solution {
    // 以长度为6的数组，比如要计算index为2的值，其值为arr[0] * arr[1] * arr[3] * arr[4] * arr[5]
    // 亦即其前缀数组的积乘以后缀数组的积。
    // 想象我们可以创建两个额外数组，其中一个存储所有到当前位置的前缀之积，另一个存储后缀之积。
    // 创建前缀数组时，我们只需从前到后遍历一次；创建后缀数组时，只需反向遍历一次。
    // 此题的挑战项目为：使用O(1)空间（不包含返回结果）
    // 那么我们可以将前缀数组与后缀数组直接融合到结果数组上，即：
    // 首先从前到后遍历一次，使用前缀之积填充结果，再从后到前遍历一次，在之前结果的基础上乘以后缀之积，
    // 注意偏移即可。
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        // 使用至当前位置的前缀之积填充结果数组
        let mut prefix_product = 1;
        for idx in 0..nums.len() - 1 {
            prefix_product *= nums[idx];
            result[idx + 1] *= prefix_product;
        }
        let mut postfix_product = 1;
        // 使用至当前位置的后缀之积更新结果数组
        for idx in (1..nums.len()).rev() {
            postfix_product *= nums[idx];
            result[idx - 1] *= postfix_product;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}
