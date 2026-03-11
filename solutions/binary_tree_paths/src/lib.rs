use std::cell::RefCell;
use std::rc::Rc;

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

pub fn inspect_node(node: Rc<RefCell<TreeNode>>, result: &mut Vec<String>, mut path: String) {
    let left = &node.borrow().left;
    let right = &node.borrow().right;

    path.push_str(&node.borrow().val.to_string());

    if left.is_none() && right.is_none() {
        result.push(path.clone());
    } else {
        path.push_str("->");
    }

    if let Some(left) = &node.borrow().left {
        inspect_node(left.clone(), result, path.clone());
    }

    if let Some(right) = &node.borrow().right {
        inspect_node(right.clone(), result, path.clone());
    }
}

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    if root.is_none() {
        return Vec::new();
    }

    let node = root.unwrap();
    let mut result: Vec<String> = Vec::new();

    inspect_node(node, &mut result, String::new());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_paths() {
        // Create tree: [1,2,3,null,5]
        //       1
        //      / \
        //     2   3
        //      \
        //       5

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        let expected = vec!["1->2->5".to_string(), "1->3".to_string()];
        let result = binary_tree_paths(root);

        assert_eq!(result, expected);
    }
}
