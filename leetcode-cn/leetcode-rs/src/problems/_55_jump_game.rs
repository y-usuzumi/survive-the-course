struct Solution;

impl Solution {
    /// 本解法的思路是从右往左遍历，其中continuous_cants变量记录了到当前位置为止一共有多少个连续的位置
    /// 无法走到终点，如：
    /// [3, 2, 1, 0, 3, 1, 0, 0]
    /// 
    /// 终点直接跳过，其余位置对应的continuous_cants的值分别如下：
    /// [3, 2, 1, 0, 3, 1, 0, 0]
    ///  |  |  |  |  |  |  |
    ///  4  3  2  1  0  2  1
    /// 判断当前continuous_cants值的原理为，如果从当前位置能跳的步数超过了当前的continuous_cants的长度，
    /// 说明可以跳到下一个终点可达的位置，则把continuous_cants清零，否则无法跳过，continuous_cants加1。
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        // 从后往前至今有多少个格是无法走到终点的
        let mut continuous_cants = 0;

        // 终点不作考虑，所以从倒数第一个位置往左走
        for idx in (0..nums.len() - 1).rev() {
            if nums[idx] > continuous_cants {
                // 如果当前位置能跳的步数超过右边无法走到终点的格数，说明可以跳过
                // 这些格前进到下一个可以跳到终点的格，或者直接到达终点。
                continuous_cants = 0;
            } else {
                // 该位置能跳的步数以内的所有格都无法走到终点，所以本身也无法达到终点。
                continuous_cants += 1;
            }
        }
        // 现在在起点，如果此时continuous_cants为0,说明从起点可以跳到下一个终点可达的格，或者直接到达终点。
        continuous_cants == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
    }
}