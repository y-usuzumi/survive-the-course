// https://leetcode.com/problems/kth-ancestor-of-a-tree-node/

pub struct TreeAncestor {
    ancestor_chain_capacity: usize,
    exp_ancestor_lookup: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    // Binary lifting
    // https://www.youtube.com/watch?v=oib-XsjFa-M
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        let (ancestor_chain_capacity, lookup) = Self::populate_ancestor_lookup(&parent);
        return Self {
            ancestor_chain_capacity,
            exp_ancestor_lookup: lookup,
        };
    }

    #[inline]
    fn populate_ancestor_lookup(parent_map: &[i32]) -> (usize, Vec<Vec<i32>>) {
        let len = parent_map.len();
        let ancestor_chain_capacity = (len as f64).log2() as usize + 1;
        let mut result = vec![vec![-1; ancestor_chain_capacity]; parent_map.len()];
        for (idx, parent) in parent_map.iter().enumerate() {
            result[idx][0] = *parent;
        }

        let mut has_parent = true;
        let mut curr_level = 1;
        while has_parent && curr_level < ancestor_chain_capacity {
            has_parent = false;
            for idx in 0..len {
                if result[idx][curr_level - 1] == -1 {
                    continue;
                }
                has_parent = true;
                let ancestor = result[result[idx][curr_level - 1] as usize][curr_level - 1];
                result[idx][curr_level] = ancestor;
            }
            curr_level += 1;
        }

        return (ancestor_chain_capacity, result);
    }

    pub fn get_kth_ancestor(&self, mut node: i32, k: i32) -> i32 {
        if k >= (1 << self.ancestor_chain_capacity << 1 - 1) {
            return -1;
        }
        for j in 0..self.ancestor_chain_capacity {
            if k & (1 << j) > 0 {
                node = self.exp_ancestor_lookup[node as usize][j] as i32;
                if node == -1 {
                    return node;
                }
            }
        }
        return node;
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ta = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(ta.get_kth_ancestor(3, 1), 1);
        assert_eq!(ta.get_kth_ancestor(5, 2), 0);
        assert_eq!(ta.get_kth_ancestor(6, 3), -1);
    }

    #[test]
    fn test_2() {
        let ta = TreeAncestor::new(10, vec![-1, 0, 0, 1, 2, 0, 1, 3, 6, 1]);
        assert_eq!(ta.get_kth_ancestor(8, 6), -1);
        assert_eq!(ta.get_kth_ancestor(9, 7), -1);
        assert_eq!(ta.get_kth_ancestor(1, 1), 0);
        assert_eq!(ta.get_kth_ancestor(2, 5), -1);
        assert_eq!(ta.get_kth_ancestor(4, 2), 0);
        assert_eq!(ta.get_kth_ancestor(7, 3), 0);
        assert_eq!(ta.get_kth_ancestor(3, 7), -1);
        assert_eq!(ta.get_kth_ancestor(9, 6), -1);
        assert_eq!(ta.get_kth_ancestor(3, 5), -1);
        assert_eq!(ta.get_kth_ancestor(8, 8), -1);
    }
}
