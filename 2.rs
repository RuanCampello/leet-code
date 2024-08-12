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
    let mut head = ListNode::new(0);
    let mut current = &mut head;
    let mut c = 0;

    let mut l1 = l1;
    let mut l2 = l2;

    while l1.is_some() || l2.is_some() || c != 0 {
        let mut sum = c;

        // if there's something in l1, sum it and pass to next value
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }
        c = sum / 10; // carry of to the next iteration
        let d = sum % 10; // get the digit if it pass 9

        // add it to the current node
        current.next = Some(Box::new(ListNode::new(d)));
        current = current.next.as_mut().unwrap();
    }
    //return the next node
    head.next
}

fn main() {
    let node3 = ListNode::new(3);
    let node4 = ListNode {
        val: 4,
        next: Some(Box::new(node3)),
    };
    let l1 = ListNode {
        val: 2,
        next: Some(Box::new(node4)),
    };

    let node4 = ListNode::new(4);
    let node6 = ListNode {
        val: 6,
        next: Some(Box::new(node4)),
    };
    let l2 = ListNode {
        val: 5,
        next: Some(Box::new(node6)),
    };

    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    let mut node = result;
    while let Some(n) = node {
        print!("{} ", n.val);
        node = n.next;
    }
}
