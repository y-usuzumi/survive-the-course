// https://leetcode.com/problems/redundant-connection/

pub struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ds = vec![-1; edges.len() + 1];
        let mut result = Vec::new();
        for edge in edges {
            let mut source_root_idx = edge[0];
            while ds[source_root_idx as usize] != -1 {
                source_root_idx = ds[source_root_idx as usize];
            }

            let mut target_root_idx = edge[1];
            while ds[target_root_idx as usize] != -1 {
                target_root_idx = ds[target_root_idx as usize];
            }
            if source_root_idx == target_root_idx {
                result.push(edge);
            } else {
                ds[target_root_idx as usize] = source_root_idx;
            }
        }
        return result[result.len() - 1].clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
