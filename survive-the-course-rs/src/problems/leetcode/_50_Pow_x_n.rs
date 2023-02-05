// https://leetcode.com/problems/powx-n/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            if n != 0 {
                return 0.0;
            }
            panic!("0 ** 0");
        }
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == -1 {
            return 1.0 / x;
        }

        let result_sub2 = Self::my_pow(x, n / 2);
        return result_sub2 * result_sub2 * Self::my_pow(x, n % 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
