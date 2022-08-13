use std::vec::Vec;

#[derive(Debug)]
pub struct Queue<T: Clone> {
    queue: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, value: T) {
        self.queue.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if self.queue.is_empty() {
            Err("Queue is empty.")
        } else {
            Ok(self.queue.remove(0))
        }
    }

    pub fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("Queue is empty."),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}
