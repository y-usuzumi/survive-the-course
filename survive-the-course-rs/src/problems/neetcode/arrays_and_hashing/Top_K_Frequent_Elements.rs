// https://leetcode.com/problems/top-k-frequent-elements/
use std::collections::{BinaryHeap, HashMap};

pub trait Solution {
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    // HashMap + max heap
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *hm.entry(num).or_default() += 1;
        }
        let mut heap = BinaryHeap::with_capacity(hm.len());
        let mut result = Vec::with_capacity(hm.len());
        for (k, v) in hm {
            heap.push((v, k));
        }
        for _ in 0..k {
            result.push(heap.pop().unwrap().1);
        }
        result
    }
}

impl Solution for Solution2 {
    // Bucket sort变种，index为出现次数，值为出现该次数的数字（可以为多个）
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for num in &nums {
            *hm.entry(*num).or_default() += 1;
        }
        let mut bucket: Vec<Vec<i32>> =
            vec![Vec::with_capacity(hm.len()); nums.len() - hm.len() + 1];
        for (k, v) in hm {
            // index: 出现次数-1 value: 出现该次数的数字
            bucket[v as usize - 1].push(k);
        }

        let mut result = Vec::with_capacity(k as usize);
        let mut count = 0;
        for idx in (0 as usize..bucket.len() as usize).rev() {
            for num in &bucket[idx] {
                result.push(*num);
                count += 1;
                if count == k {
                    return result;
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use test_util::assert_eq_ignore_order;

    use super::*;

    mod solution1 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq_ignore_order(
                Solution1::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
                vec![1, 2],
            );
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq_ignore_order(
                Solution2::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
                vec![1, 2],
            );
        }
    }
}
