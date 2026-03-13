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
    let mut sub_queue = VecDeque::new();

    queue.push_back(root);

    let mut layer: u32 = 1;

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap().unwrap();

        match &node.borrow().left {
            Some(left) => sub_queue.push_back(Some(left.clone())),
            None => sub_queue.push_back(None),
        }

        match &node.borrow().right {
            Some(right) => sub_queue.push_back(Some(right.clone())),
            None => sub_queue.push_back(None),
        }

        if queue.is_empty() {
            let layer_nodes_count = (2 as i32).pow(layer);
            let middle = (layer_nodes_count / 2) as usize;
            let node_values: Vec<Option<i32>> = sub_queue
                .iter()
                .map(|maybe_node| match maybe_node {
                    Some(node) => Some(node.borrow().val),
                    None => None,
                })
                .collect();

            let nodes_count = node_values.len();

            for (i, left_val) in node_values.iter().enumerate() {
                if i == middle {
                    break;
                }

                let comparing_node_index = nodes_count - i - 1;
                let right_val = node_values.get(comparing_node_index).unwrap();

                if left_val != right_val {
                    return false;
                }
            }

            layer += 1;

            sub_queue.retain(|item| item.is_some());
            queue.append(&mut sub_queue);
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
