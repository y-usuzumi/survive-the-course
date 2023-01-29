#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn singleton(val: T) -> Self {
        Self {
            val: val,
            left: None,
            right: None,
        }
    }
    fn from_vec(mut arr: Vec<Option<T>>) -> Option<Box<Self>> {
        if arr.is_empty() || arr[0].is_none() {
            return None;
        }

        arr.reverse();

        let mut root = Self::singleton(arr.pop().unwrap().unwrap());
        let mut curr_layer = vec![Some(Box::new(&mut root))];
        loop {
            let mut next_layer = vec![];
            for parent in curr_layer {
                if let Some(parent) = parent {
                    match arr.pop() {
                        Some(Some(val)) => {
                            parent.left = Some(Box::new(Self::singleton(val)));
                        }
                        Some(None) => {}
                        None => {
                            return Some(Box::new(root));
                        }
                    }

                    match arr.pop() {
                        Some(Some(val)) => {
                            parent.right = Some(Box::new(Self::singleton(val)));
                        }
                        Some(None) => {}
                        None => {
                            return Some(Box::new(root));
                        }
                    }

                    let left = parent.left.as_mut().map(|node| Box::new(node.as_mut()));
                    let right = parent.right.as_mut().map(|node| Box::new(node.as_mut()));
                    next_layer.push(left);
                    next_layer.push(right);
                } else {
                    if arr.pop().is_none() {
                        return Some(Box::new(root));
                    }
                    if arr.pop().is_none() {
                        return Some(Box::new(root));
                    }
                }
            }
            curr_layer = next_layer;
        }
    }
}

#[cfg(test)]
mod tests {
    use test_util::option_vec;

    use super::*;

    #[test]
    fn test_option_vec() {
        assert_eq!(option_vec![1], vec![Some(1)]);
        assert_eq!(option_vec![null], vec![None] as Vec<Option<i32>>);
        assert_eq!(
            option_vec![1, 2, null, 3],
            vec![Some(1), Some(2), None, Some(3)]
        )
    }

    #[test]
    fn test_binary_tree_from_vec() {
        let data = option_vec![1, 2, 3, null, 5, 6];
        let tree = TreeNode::from_vec(data);
        let tree = tree.unwrap();
        assert_eq!(tree.val, 1);
        let left = tree.left.unwrap();
        let right = tree.right.unwrap();
        assert_eq!(left.val, 2);
        assert_eq!(right.val, 3);
        let leftright = left.right.unwrap();
        let rightleft = right.left.unwrap();
        assert!(left.left.is_none());
        assert_eq!(leftright.val, 5);
        assert_eq!(rightleft.val, 6);
        assert!(right.right.is_none());
    }
}
