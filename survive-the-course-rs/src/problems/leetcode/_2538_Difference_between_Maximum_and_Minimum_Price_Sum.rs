// https://leetcode.com/problems/difference-between-maximum-and-minimum-price-sum/description/

use std::collections::HashSet;

pub struct Solution;

// TODO: Review
impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push(edge[1] as usize);
            g[edge[1] as usize].push(edge[0] as usize);
        }

        fn helper(
            g: &Vec<Vec<usize>>,
            prices: &Vec<i32>,
            root: usize,
            parent: usize,
            global_max: &mut i64,
        ) -> (i64, i64) {
            let children = &g[root];
            if children.is_empty() {
                return (0, 0);
            }
            if children.len() == 1 && parent != usize::MAX {
                return (prices[root] as i64, 0);
            }
            let root_price = prices[root] as i64;
            let mut child_max_with_tip = 0;
            let mut child_max_without_tip = 0;
            let mut is_first = true;
            for &child in children {
                if child == parent {
                    continue;
                }
                let (max_with_tip, max_without_tip) = helper(g, prices, child, root, global_max);
                if is_first {
                    child_max_with_tip = max_with_tip;
                    child_max_without_tip = max_without_tip;
                    is_first = false;
                } else {
                    *global_max = (*global_max)
                        .max(child_max_with_tip + root_price + max_without_tip)
                        .max(child_max_without_tip + root_price + max_with_tip);
                }

                if max_with_tip > child_max_with_tip {
                    child_max_with_tip = max_with_tip;
                }
                if max_without_tip > child_max_without_tip {
                    child_max_without_tip = max_without_tip;
                }
            }
            *global_max = (*global_max)
                .max(child_max_with_tip)
                .max(child_max_without_tip + root_price);
            return (
                child_max_with_tip + root_price,
                child_max_without_tip + root_price,
            );
        }

        let mut global_max = 0;
        helper(&g, &price, 0, usize::MAX, &mut global_max);

        return global_max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
