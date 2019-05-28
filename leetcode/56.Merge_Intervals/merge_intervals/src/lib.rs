pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 由于需要使用intervals中第一个区间作为基准，因此对空做特殊判断。
        if intervals.len() == 0 {
            return Vec::new();
        }

        // 以左值为基准进行排序。
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        // 结果集
        let mut result = Vec::new();

        // 取出第0个区间。
        let mut curr_intv = intervals[0].clone();

        // 遍历剩下的区间
        for intv in intervals.iter().skip(1) {
            if intv[0] > curr_intv[1] {  // 不重合，
                // 则把之前合并的结果存入结果集，
                result.push(curr_intv);
                // 然后以当前区间为基准继续。
                curr_intv = intv.clone();
            } else {
                // 如果重合，如：[1, 4] 和 [2, 5]，
                // 则取 (4) 和 (5) 中更大的作为合并结果的右值：
                // [1, 5]。
                curr_intv[1] = curr_intv[1].max(intv[1]);
            }
        }
        // 最后一个结果还没有存入结果集。
        result.push(curr_intv);
        // 返回最终结果
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
