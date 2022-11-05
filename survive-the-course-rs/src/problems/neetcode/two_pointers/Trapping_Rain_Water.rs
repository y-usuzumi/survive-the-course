// https://leetcode.com/problems/trapping-rain-water/

pub struct Solution;

impl Solution {
    // 双指针
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut idxl = 0;
        let mut heightl = height[idxl];
        let mut idxr = height.len() - 1;
        let mut heightr = height[idxr];
        let mut result = 0;
        while idxl < idxr {
            // 矮的一边更可能漏水
            if heightl < heightr {
                // 如果相邻格更矮，能装的水量为边缘高度减去相邻格高度
                if height[idxl + 1] < heightl {
                    result += heightl - height[idxl + 1];
                } else {
                    // 相邻格更高，则更新左边缘高度，因为越往内侧越可能装更多水
                    heightl = height[idxl + 1];
                }
                idxl += 1;
            } else {
                // 矮的一边更可能漏水
                // 如果相邻格更矮，能装的水量为边缘高度减去相邻格高度
                if height[idxr - 1] < heightr {
                    result += heightr - height[idxr - 1];
                } else {
                    // 相邻格更高，则更新右边缘高度，因为越往内侧越可能装更多水
                    heightr = height[idxr - 1];
                }
                idxr -= 1;
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
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
