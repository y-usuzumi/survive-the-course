// https://leetcode.com/problems/student-attendance-record-ii/

pub struct Solution;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_000 + 7;
        let n = n as usize;
        let mut result_table = vec![vec![vec![0; n]; 3]; 2];
        result_table[0][0][0] = 1; // P
        result_table[1][0][0] = 1; // A
        result_table[0][1][0] = 1; // L

        for day in 1..n {
            for absent_idx in 0..2 {
                // P today
                for late_idx in 0..3 {
                    result_table[absent_idx][0][day] += result_table[absent_idx][late_idx][day - 1];
                    result_table[absent_idx][0][day] %= MOD;
                }

                // A today
                if absent_idx > 0 {
                    for late_idx in 0..3 {
                        result_table[absent_idx][0][day] +=
                            result_table[absent_idx - 1][late_idx][day - 1];
                        result_table[absent_idx][0][day] %= MOD;
                    }
                }

                // L today
                for late_idx in 1..3 {
                    result_table[absent_idx][late_idx][day] +=
                        result_table[absent_idx][late_idx - 1][day - 1];
                    result_table[absent_idx][late_idx][day] %= MOD;
                }
            }
        }

        let mut result = 0;
        for absent_idx in 0..2 {
            for late_idx in 0..3 {
                result += result_table[absent_idx][late_idx][n - 1];
                result %= MOD;
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
        assert_eq!(Solution::check_record(2), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}
