// Integration test for queue module
// This file tests the queue as an external user would

// Type alias to avoid repeating the full path
use probe::queue::Queue;

#[test]
fn test_enqueue_and_dequeue() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_peek() {
    let mut queue = Queue::new();
    assert!(queue.peek().is_none());
    
    queue.enqueue(42);
    assert_eq!(*queue.peek().unwrap(), 42);
    
    queue.enqueue(100);
    assert_eq!(*queue.peek().unwrap(), 42); // Should still peek at first item
}

#[test]
fn test_empty_queue() {
    let mut queue= Queue::<i32>::new();
    assert_eq!(queue.dequeue(), None);
    assert!(queue.peek().is_none());
} 