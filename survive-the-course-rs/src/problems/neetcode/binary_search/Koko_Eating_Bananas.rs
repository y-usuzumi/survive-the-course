// https://leetcode.com/problems/koko-eating-bananas/

pub struct Solution;

impl Solution {
    // Binary search. Take the greater of the two pointers (left) because the
    // less pointer may not satisfy the hours
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();

        while left <= right {
            let mid = (left + right) / 2;
            if Self::satisfies(&piles, mid, h) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }

    #[inline]
    // If accumulated hours so far exceeds h, break early to avoid overflow
    fn satisfies(piles: &[i32], k: i32, h: i32) -> bool {
        let mut hours = 0;
        for pile in piles {
            let pile_hours = pile / k + if pile % k > 0 { 1 } else { 0 } as i32;
            hours += pile_hours;
            if hours > h {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::min_eating_speed(vec![312884470], 312884469), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::min_eating_speed(vec![1, 1, 1, 999999999], 10),
            142857143
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
    }
}
