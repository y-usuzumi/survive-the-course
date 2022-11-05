// https://leetcode.com/problems/container-with-most-water/

pub struct Solution;

impl Solution {
    // 双指针
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let mut result = 0;
        let mut idxl = 0;
        let mut idxr = height.len() - 1;
        while idxl < idxr {
            let heightl = height[idxl];
            let heightr = height[idxr];
            result = result.max((idxr - idxl) as i32 * heightl.min(heightr));
            // 我们希望一直保留其中较高的一端，这样才更可能得到更大的面积
            if heightl < heightr {
                idxl += 1;
            } else {
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
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
