use std::fmt::Display;

struct StackNode<T: Clone + Display> {
    data: T,
    previous_node: Option<Box<StackNode<T>>>,
}

impl<T: Clone + Display> StackNode<T> {
    fn new(data: T, previous_node: Option<Box<StackNode<T>>>) -> Self {
        Self {
            data,
            previous_node,
        }
    }
}

pub struct Stack<T: Clone + Display> {
    top: Option<Box<StackNode<T>>>,
}

impl<T: Clone + Display> Stack<T> {
    pub fn new() -> Self {
        Self { top: None }
    }

    pub fn insert_data(&mut self, data: T) {
        let new_node = Box::new(StackNode::new(data.clone(), self.top.take()));
        self.top = Some(new_node);
    }

    pub fn pop_data(&mut self) -> Option<T> {
        match self.top.take() {
            Some(mut node) => {
                self.top = node.previous_node.take();
                Some(node.data)
            }
            None => None,
        }
    }
}