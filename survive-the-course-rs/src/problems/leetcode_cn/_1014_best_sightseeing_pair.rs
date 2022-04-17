// https://leetcode-cn.com/problems/best-sightseeing-pair/

pub struct Solution;

/// 本解法为我自己的思路。
/// 仔细分析一下两个景点的总得分公式：
/// values[i, j] = values[i] + values[j] + i - j,即两景点得分之和减去两景点间的距离。
/// 对于任何一个可能的右景点，我们可以定义某个左景点的价值为values[i] - (j - i)，即它的
/// 原始得分减去两景点间的距离。
/// 我们假设右景点的指针从左往右行进，不论左景点选哪个，他们之间的相对分数都是固定的。比如：
/// [3, 1, 4, 5]
/// 当右景点为4时，左景点3的价值为3-2 = 1,左景点1的价值为1-1 = 0，之间相差为1,
/// 当右景点为5时，左景点3的价值为3-3 = 0,左景点1的价值为1-2 = -1，之间相差仍为1。
/// 原因在于每往右一格，所有的左景点对右景点的距离同步增加1。
/// 所以我们可以从已经路过的景点中随时挑选出价值最大的一个，每路过一个新节点，可以计算出它的价
/// 值并随时更新最大价值。
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        // 当前左景点的最大价值
        let mut left_val = values[0];
        // 最大的景点总得分
        let mut max_sum_of_vals = 0;
        for val in values.into_iter().skip(1) {
            // 每走一步，价值最大的左景点离得更远了，价值减一
            left_val -= 1;
            // 更新总得分
            if left_val + val > max_sum_of_vals {
                max_sum_of_vals = left_val + val;
            }
            // 现在右景点即将变成左景点，计算其价值并更新最大价值变量
            if val > left_val {
                left_val = val;
            }
        }
        max_sum_of_vals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]),
            11
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
