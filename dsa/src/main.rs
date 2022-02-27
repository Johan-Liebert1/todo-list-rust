use data::linked_list::LinkedList;

mod data;
fn main() {
    let mut linked_list = LinkedList { start: None };

    linked_list.insert(5);
    linked_list.insert(10);
    linked_list.insert(15);
    linked_list.insert(20);
    linked_list.insert(25);
    linked_list.insert(30);
    linked_list.insert(35);

    linked_list.traverse();
}
