pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max_start_idx, mut max_end_idx) = (0, 0);
        let (mut start_idx, mut end_idx) = (0, 0);
        let mut current_sum = 0;
        loop {
            current_sum += start_idx;
            if current_sum < 0 {
                current_sum = 0;
                start_idx = end_idx+1;
                end_idx = start_idx;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
