// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_val = prices[0];
        let mut max_profit = 0;
        for val in prices.into_iter().skip(1) {
            if val < min_val {
                min_val = val;
            } else {
                max_profit = max(max_profit, val - min_val);
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
