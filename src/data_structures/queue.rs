use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { elements: VecDeque::new() }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn show(&self) where T: std::fmt::Debug {
        println!("Cola (frente -> final): {:?}", self.elements);
    }
}