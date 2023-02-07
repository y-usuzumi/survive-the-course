// https://leetcode.com/problems/insert-interval/

pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut merged = false;
        for interval in intervals {
            if merged || interval[1] < new_interval[0] {
                result.push(interval);
            } else if interval[0] > new_interval[1] {
                if !merged {
                    result.push(new_interval.clone());
                    merged = true;
                }
                result.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        }
        if !merged {
            result.push(new_interval);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
