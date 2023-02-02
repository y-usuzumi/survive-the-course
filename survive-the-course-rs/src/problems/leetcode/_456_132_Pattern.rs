// https://leetcode.com/problems/132-pattern/

pub struct Solution1;

impl Solution1 {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<Vec<i32>> = vec![vec![i32::MAX, i32::MIN]];
        for num in nums {
            for range in stack.iter().rev() {
                if num > range[0] && num < range[1] {
                    return true;
                }
            }

            let len = stack.len();

            let last = stack.get_mut(len - 1).unwrap();
            if num < last[0] {
                if last[0] < last[1] {
                    stack.push(vec![num, i32::MIN]);
                } else {
                    last[0] = num;
                }
            } else if num > last[1] {
                last[1] = num;
                let mut idx = len - 1;
                while idx > 0 {
                    if stack[idx][1] >= stack[idx - 1][1] {
                        stack[idx - 1][0] = stack[idx][0];
                        stack[idx - 1][1] = stack[idx][1];
                        stack.pop();
                        idx -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
        return false;
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut min = i32::MIN;
        for num in nums.into_iter().rev() {
            if num < min {
                return true;
            }
            while let Some(&low) = stack.last() {
                if num > low {
                    stack.pop();
                    min = min.max(low);
                } else {
                    break;
                }
            }
            stack.push(num);
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution1::find132pattern(vec![1, 3, -1, 8, -7, -3, 6]));
    }

    #[test]
    fn test_2() {
        assert!(Solution2::find132pattern(vec![1, 3, -1, 8, -7, -3, 6]));
    }
}
