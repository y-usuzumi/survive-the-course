use std::iter::FromIterator;
use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        nums.iter().enumerate().for_each(|(i, v)| {
            if !hm.contains_key(v)  {
                hm.insert(*v, Vec::with_capacity(2));
            }
            hm.get_mut(v).unwrap().push(i as i32);
        });
        println!("{:?}", hm);
        for (i, v) in nums.iter().enumerate() {
            println!("{}: {}", i, v);
            match hm.get(&(target - v)) {
                Some(n) => {
                    if target - v == *v && n.len() < 2 {
                        continue;
                    }
                    return vec![i as i32, n[n.len() - 1]]
                },
                None => {},
            }
        }
        panic!("Impossible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        let result = Solution::two_sum(vec![2, 7, 11, 5], 9);
        assert_eq!(result, vec![0, 1]);
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }
}
