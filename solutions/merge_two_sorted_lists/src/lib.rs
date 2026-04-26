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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() || l2.is_some() {
        match (&l1, &l2) {
            (Some(item1), Some(item2)) => {
                if item1.val < item2.val {
                    tail.next = Some(Box::new(ListNode::new(item1.val)));
                    tail = tail.next.as_mut().unwrap();
                    l1 = item1.next.clone();
                } else if item1.val > item2.val {
                    tail.next = Some(Box::new(ListNode::new(item2.val)));
                    tail = tail.next.as_mut().unwrap();
                    l2 = item2.next.clone();
                } else if item1.val == item1.val {
                    tail.next = Some(Box::new(ListNode::new(item1.val)));
                    tail = tail.next.as_mut().unwrap();
                    tail.next = Some(Box::new(ListNode::new(item2.val)));
                    tail = tail.next.as_mut().unwrap();
                    l1 = item1.next.clone();
                    l2 = item2.next.clone();
                }
            }
            (Some(item1), None) => {
                tail.next = Some(Box::new(ListNode::new(item1.val)));
                tail = tail.next.as_mut().unwrap();
                l1 = item1.clone().next;
            }
            (None, Some(item2)) => {
                tail.next = Some(Box::new(ListNode::new(item2.val)));
                tail = tail.next.as_mut().unwrap();
                l2 = item2.clone().next;
            }
            (None, None) => {
                break;
            }
        }
    }

    dummy.next
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
    fn case1() {
        let l1 = vec_to_list(vec![2, 3, 4]);
        let l2 = vec_to_list(vec![1, 2, 3]);
        let result = merge_two_lists(l1, l2);
        let expected = vec![1, 2, 2, 3, 3, 4];

        assert_eq!(result, vec_to_list(expected));
    }

    #[test]
    fn case2() {
        let l1 = vec_to_list(vec![5]);
        let l2 = vec_to_list(vec![1, 2, 4]);
        let result = merge_two_lists(l1, l2);
        let expected = vec![1, 2, 4, 5];

        assert_eq!(result, vec_to_list(expected));
    }
}
