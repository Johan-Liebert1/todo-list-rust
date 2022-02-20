#[derive(Debug)]
pub struct ListNode {
    pub next: Option<Box<ListNode>>,
    pub value: i32,
}

pub struct LinkedList {
    pub start: Option<Box<ListNode>>,
}

impl LinkedList {
    fn insert(&mut self, value: i32) {
        let mut ptr: &mut Box<ListNode> = match &self.start {
            Some(node) => node,
            None => {
                // first node to be inserted
                let new_node = ListNode { value, next: None };
                self.start = Some(Box::new(new_node));
                return;
            }
        };

        let mut last_node = loop {
            ptr = match &ptr.next {
                Some(node) => &mut node,
                None => break,
            };

            let new_node = ListNode { value, next: None };
            ptr.next = Some(Box::new(new_node));
        };
    }
}
