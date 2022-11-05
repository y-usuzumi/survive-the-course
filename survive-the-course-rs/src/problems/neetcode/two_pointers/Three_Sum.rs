// https://leetcode.com/problems/3sum/

pub struct Solution;

impl Solution {
    // 先排序，然后固定左边元素，剩下两个参照Two Sum
    // 注意如何排除重复答案
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();
        for idx1 in 0..nums.len() - 2 {
            // 如果最左数字已经超过0,则剩下的根本不用看
            if nums[idx1] > 0 {
                return result;
            }
            // 如果当前数字和其左边相邻数字相同，则不用看，因为
            // 所有的可能性已经被左边数字包含
            if idx1 > 0 && nums[idx1] == nums[idx1 - 1] {
                continue;
            }
            let mut idx2 = idx1 + 1;
            // 如果左边两数之和大于0,则右边的不用看
            if nums[idx1] + nums[idx2] > 0 {
                continue;
            }
            let mut idx3 = nums.len() - 1;
            while idx2 < idx3 {
                match (nums[idx1] + nums[idx2] + nums[idx3]).cmp(&0) {
                    std::cmp::Ordering::Less => {
                        idx2 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        idx3 -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[idx1], nums[idx2], nums[idx3]]);
                        idx2 += 1;
                        // 之所以最右下标可以左移，是因为若只有中间下标右移而最右下标不动
                        // 且答案满足条件，那一定与上一个答案重复
                        idx3 -= 1;
                        while nums[idx2] == nums[idx2 - 1] && idx2 < idx3 {
                            idx2 += 1;
                        }
                    }
                }
            }
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
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
