use forge::structures::Queue;

#[test]
fn test_enqueue() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(64);
    assert_eq!(queue.is_empty(), false);
}

#[test]
fn test_dequeue() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(32);
    queue.enqueue(64);
    assert_eq!(queue.dequeue(), Ok(32));
}

#[test]
fn test_peek() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(8);
    queue.enqueue(16);
    assert_eq!(queue.peek(), Ok(8));
}

#[test]
fn test_size() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(8);
    queue.enqueue(16);
    assert_eq!(2, queue.len());
}
