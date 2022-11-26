// https://leetcode.com/problems/student-attendance-record-i/

pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent_days = 0;
        let mut late_consecutive_days = 0;
        for ch in s.chars() {
            match ch {
                'A' => {
                    absent_days += 1;
                    if absent_days > 1 {
                        return false;
                    }
                    late_consecutive_days = 0;
                }
                'L' => {
                    late_consecutive_days += 1;
                    if late_consecutive_days >= 3 {
                        return false;
                    }
                }
                'P' => {
                    late_consecutive_days = 0;
                }
                _ => panic!("Impossible"),
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
        assert!(Solution::check_record("PPALLP".into()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_record("PPALLL".into()));
    }
}
