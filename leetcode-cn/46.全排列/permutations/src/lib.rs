struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![];
        }
        if nums.len() == 1 {
            return vec![nums];
        }
        let (head, tail) = (nums[0], Vec::from(&nums[1..]));
        let tail_permute = Solution::permute(tail);
        let mut result = Vec::new();
        for tail_row in tail_permute {
            for idx in 0..=tail_row.len() {
                let mut left = Vec::from(&tail_row[..idx]);
                left.push(head);
                left.extend(&tail_row[idx..]);
                result.push(left);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::permute(vec![1, 2, 3]));
    }
}
