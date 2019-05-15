pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return Vec::new();
        }
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = Vec::new();
        let mut curr_intv = sorted_intervals[0].clone();
        for intv in sorted_intervals.iter().skip(1) {
            if intv[0] > curr_intv[1] {
                result.push(curr_intv);
                curr_intv = intv.clone();
            } else {
                curr_intv[1] = curr_intv[1].max(intv[1]);
            }
        }
        result.push(curr_intv);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_for_sample_test_1() {
        assert_eq!(
            Solution::merge(vec![
                vec![1,3],
                vec![2,6],
                vec![8,10],
                vec![15,18]
            ]),
            vec![
                vec![1,6],
                vec![8,10],
                vec![15,18]
            ]
        );
    }

    #[test]
    fn it_works_for_sample_test_2() {
        assert_eq!(
            Solution::merge(vec![
                vec![1,4],
                vec![1,5]
            ]),
            vec![
                vec![1,5]
            ]
        );
    }
}
