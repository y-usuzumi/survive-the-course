// https://leetcode.com/problems/car-pooling/

use std::{cmp::Reverse, collections::BinaryHeap};

pub trait Solution {
    fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        trips.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut h: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut curr_passengers = 0;
        for trip in trips {
            while !h.is_empty() {
                if h.peek().unwrap().0 .0 <= trip[1] {
                    let prev = h.pop().unwrap();
                    curr_passengers -= prev.0 .1;
                } else {
                    break;
                }
            }
            curr_passengers += trip[0];
            if curr_passengers > capacity {
                return false;
            }
            h.push(Reverse((trip[2], trip[0])));
        }

        return true;
    }
}

impl Solution for Solution2 {
    fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut arr = vec![0; 1001];

        for trip in trips {
            arr[trip[1] as usize] += trip[0];
            arr[trip[2] as usize] -= trip[0];
        }

        let mut curr_passengers = 0;
        for num in arr {
            curr_passengers += num;
            if curr_passengers > capacity {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution1 {
        use super::*;

        #[test]
        fn test_1() {
            assert!(!Solution1::car_pooling(
                vec![vec![2, 1, 5], vec![3, 3, 7]],
                4
            ));
        }

        #[test]
        fn test_2() {
            assert!(Solution1::car_pooling(
                vec![vec![2, 1, 5], vec![3, 3, 7]],
                5
            ));
        }

        #[test]
        fn test_3() {
            assert!(Solution1::car_pooling(
                vec![vec![2, 1, 5], vec![3, 5, 7]],
                3
            ));
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert!(!Solution2::car_pooling(
                vec![vec![2, 1, 5], vec![3, 3, 7]],
                4
            ));
        }

        #[test]
        fn test_2() {
            assert!(Solution2::car_pooling(
                vec![vec![2, 1, 5], vec![3, 3, 7]],
                5
            ));
        }

        #[test]
        fn test_3() {
            assert!(Solution2::car_pooling(
                vec![vec![2, 1, 5], vec![3, 5, 7]],
                3
            ));
        }
    }
}
