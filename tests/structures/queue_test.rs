use forge::queue;
use forge::structures::Queue;

#[test]
fn enqueue() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(64);
    assert_eq!(queue.is_empty(), false);
}

#[test]
fn dequeue() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(32);
    queue.enqueue(64);
    assert_eq!(queue.dequeue(), Ok(32));
}

#[test]
fn peek() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(8);
    queue.enqueue(16);
    assert_eq!(queue.peek(), Ok(8));
}

#[test]
fn size() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(8);
    queue.enqueue(16);
    assert_eq!(queue.len(), 2);
}

#[test]
fn queue_macro() {
    let mut queue = queue![17, 1, 5];
    queue.enqueue(8);
    queue.enqueue(16);
    assert_eq!(queue.len(), 5);
}
