// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn helper(output: &mut String, root: Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = root {
                let noderef = node.borrow();
                output.push_str(&noderef.val.to_string());
                output.push('(');
                helper(output, noderef.left.clone());
                output.push(',');
                helper(output, noderef.right.clone());
                output.push(')');
            }
        }

        let mut output = String::new();
        helper(&mut output, root);

        return output;
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(input: &Vec<char>, cursor: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            if *cursor >= input.len() {
                return None;
            }

            let mut val_buf = String::new();
            while input[*cursor].is_numeric() || input[*cursor] == '-' {
                val_buf.push(input[*cursor]);
                *cursor += 1;
            }
            if val_buf.is_empty() {
                return None;
            }

            let mut node = TreeNode::new(val_buf.parse().unwrap());
            assert_eq!(input[*cursor], '(');
            *cursor += 1;
            let left_node = helper(input, cursor);
            node.left = left_node;
            assert_eq!(input[*cursor], ',');
            *cursor += 1;
            let right_node = helper(input, cursor);
            node.right = right_node;
            assert_eq!(input[*cursor], ')');
            *cursor += 1;
            return Some(Rc::new(RefCell::new(node)));
        }

        return helper(&data.chars().collect(), &mut 0);
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let codec = Codec::new();

        let mut tree = TreeNode::new(3);
        tree.left = Some(Rc::new(RefCell::new({
            let mut tree = TreeNode::new(4);
            tree.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
            tree
        })));
        tree.right = Some(Rc::new(RefCell::new({
            let mut tree = TreeNode::new(6);
            tree.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
            tree
        })));
        let tree = Some(Rc::new(RefCell::new(tree)));

        let data = {
            let data: String = codec.serialize(tree);
            println!("{}", data);
            data
        };

        let tree = codec.deserialize(data.clone());
        let data2 = codec.serialize(tree);
        assert_eq!(data, data2);
    }
}
