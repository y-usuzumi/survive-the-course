use std::cmp;

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        // 2 <= cost.length <= 1000

        for idx in 2..cost.len() {
            cost[idx] += cmp::min(cost[idx-1], cost[idx-2]);
        }

        cmp::min(cost[cost.len()-1], cost[cost.len()-2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10,15,20]), 15);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]), 6);
    }
}