use std::io::Write;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub next: Option<Box<ListNode>>,
    pub value: i32,
}

pub struct LinkedList {
    pub start: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn insert(&mut self, value: i32) {
        let mut ptr: &mut Box<ListNode> = match &mut self.start {
            Some(node) => node,
            None => {
                // first node to be inserted
                let new_node = ListNode { value, next: None };
                self.start = Some(Box::new(new_node));

                println!("Inserted first node: {:?}", self.start);

                return;
            }
        };

        ptr = loop {
            unsafe {
                if let Some(i) = &mut ptr.next {
                    ptr = i;
                    // println!("i = {:?}", i);
                } else {
                    break ptr;
                }
            }
        };

        println!(
            "Inserted node: {:?} after {:?}",
            ListNode { value, next: None },
            ptr
        );

        let new_node = ListNode { value, next: None };
        ptr.next = Some(Box::new(new_node));
    }

    pub fn traverse(&self) {
        let mut ptr = &self.start;

        loop {
            ptr = match ptr {
                Some(node) => {
                    print!("{} -> ", node.value);
                    &node.next
                }
                None => break,
            }
        }

        std::io::stdout().flush().unwrap();
    }
}
