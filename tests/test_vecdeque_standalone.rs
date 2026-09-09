#[path = "../src/klib/vec.rs"]
pub mod vec;

pub use vec::Vec;

#[path = "../src/klib/vecdeque.rs"]
pub mod vecdeque;

use vecdeque::VecDeque;

#[test]
fn test_vecdeque_basic_operations() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    assert!(deque.is_empty());
    assert_eq!(deque.len(), 0);

    deque.push_back(10);
    deque.push_back(20);
    deque.push_front(5);

    assert_eq!(deque.front(), Some(&5));
    assert_eq!(deque.back(), Some(&20));
    assert_eq!(deque.len(), 3);

    assert_eq!(deque.pop_front(), Some(5));
    assert_eq!(deque.pop_front(), Some(10));
    assert_eq!(deque.pop_front(), Some(20));
    assert_eq!(deque.pop_front(), None);
    assert!(deque.is_empty());
}

#[test]
fn test_vecdeque_pop_back_with_transfer() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_front(100);
    deque.push_front(200);

    assert_eq!(deque.pop_back(), Some(100));
    assert_eq!(deque.pop_back(), Some(200));
    assert_eq!(deque.pop_back(), None);
}

#[test]
fn test_vecdeque_with_capacity() {
    let mut deque: VecDeque<i32> = VecDeque::with_capacity(16);
    deque.push_back(1);
    deque.push_back(2);
    assert_eq!(deque.len(), 2);
    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_front(), Some(2));
}

#[test]
fn test_vecdeque_non_clone_type() {
    struct NonCloneItem(u32);

    let mut deque: VecDeque<NonCloneItem> = VecDeque::new();
    deque.push_back(NonCloneItem(42));
    deque.push_front(NonCloneItem(24));

    assert_eq!(deque.pop_front().map(|x| x.0), Some(24));
    assert_eq!(deque.pop_front().map(|x| x.0), Some(42));
    assert_eq!(deque.pop_front().map(|x| x.0), None);
}

#[test]
fn test_vecdeque_iteration() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(10);
    deque.push_back(20);
    deque.push_front(5);

    let collected: std::vec::Vec<i32> = deque.iter().copied().collect();
    assert_eq!(collected, vec![5, 10, 20]);
}
