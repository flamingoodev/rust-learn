#[derive(Debug)]
pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            data: Vec::with_capacity(size),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }
    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let t = self.data.remove(0);
            Some(t)
        }
    }
    pub fn size(&mut self) -> usize {
        self.data.len()
    }
}
