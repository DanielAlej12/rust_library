use std::fmt;

#[derive(Debug)]
struct StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
    head: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(StackNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn show(&self) where T: fmt::Display {
        println!("Pila (tope -> final): {}", self);
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        
        write!(f, "[")?;
        
        while let Some(node) = current {
            write!(f, "{}", node.value)?;
            current = &node.next;
            
            if current.is_some() {
                write!(f, ", ")?;
            }
        }
        
        write!(f, "]")
    }
}