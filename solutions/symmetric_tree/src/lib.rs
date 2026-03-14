use std::cell::RefCell;
use std::collections::VecDeque;
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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = VecDeque::new();

    match root {
        Some(node) => queue.push_back((node.borrow().left.clone(), node.borrow().right.clone())),
        None => return true,
    }

    while let Some((left, right)) = queue.pop_front() {
        match (left, right) {
            (None, None) => {}
            (Some(left_node), Some(right_node)) => {
                let left_node_ref = left_node.borrow();
                let right_node_ref = right_node.borrow();

                if left_node_ref.val != right_node_ref.val {
                    return false;
                }

                queue.push_back((left_node_ref.left.clone(), right_node_ref.right.clone()));
                queue.push_back((left_node_ref.right.clone(), right_node_ref.left.clone()));
            }
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Example 1: Symmetric tree
        //     1
        //    / \
        //   2   2
        //  / \ / \
        // 3  4 4  3
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));

        assert_eq!(is_symmetric(root), true);
    }

    #[test]
    fn case2() {
        // Example 2: Asymmetric tree
        //     1
        //    / \
        //   2   2
        //    \   \
        //     3   3
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));

        assert_eq!(is_symmetric(root), false);
    }
}
