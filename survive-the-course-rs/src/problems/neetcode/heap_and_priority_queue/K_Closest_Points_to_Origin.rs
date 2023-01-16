// https://leetcode.com/problems/k-closest-points-to-origin/

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        fn distance(point: &Vec<i32>) -> i64 {
            return point[0] as i64 * point[0] as i64 + point[1] as i64 * point[1] as i64;
        }
        let mut h = BinaryHeap::with_capacity(k);
        for point in points {
            h.push((distance(&point), point));
            if h.len() > k {
                h.pop();
            }
        }

        return h.into_iter().map(|v| v.1).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
