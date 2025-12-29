use std::collections::LinkedList;
use std::collections::VecDeque;

#[test]
fn test_sequences() {
    // LinkedList
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    println!("LinkedList: {:?}", list);

    // VecDeque
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_front(2);
    println!("VecDeque: {:?}", deque);
}
