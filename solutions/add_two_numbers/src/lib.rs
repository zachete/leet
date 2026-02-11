#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut carry = 0;
    let mut current = &mut head;
    let mut p_l1 = l1;
    let mut p_l2 = l2;

    while p_l1.is_some() || p_l2.is_some() || carry != 0 {
        let x = match p_l1.take() {
            Some(node) => {
                p_l1 = node.next;

                node.val
            }
            None => 0,
        };

        let y = match p_l2.take() {
            Some(node) => {
                p_l2 = node.next;

                node.val
            }
            None => 0,
        };

        let sum = x + y + carry;

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    #[test]
    fn demo1() {
        let l1 = vec_to_list(vec![2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        let expected = vec![7, 0, 8];

        assert_eq!(result, vec_to_list(expected));
    }

    #[test]
    fn demo2() {
        let l1 = vec_to_list(vec![9]);
        let l2 = vec_to_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        let expected = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

        assert_eq!(result, vec_to_list(expected));
    }
}
