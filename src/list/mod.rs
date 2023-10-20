use std::cell::RefCell;
use std::rc::Rc;

struct ListNode<T> {
    data: T,
    next_node: Option<Rc<RefCell<ListNode<T>>>>,
    previous_node: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    fn new(
        data: T,
        previous_node: Option<Rc<RefCell<ListNode<T>>>>,
        next_node: Option<Rc<RefCell<ListNode<T>>>>,
    ) -> Self {
        Self {
            data,
            next_node,
            previous_node,
        }
    }
}

pub struct List<T> {
    first_node: Option<Rc<RefCell<ListNode<T>>>>,
    last_node: Option<Rc<RefCell<ListNode<T>>>>,
    length: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            first_node: None,
            last_node: None,
            length: 0,
        }
    }

    fn insert_last(&mut self, data: T) {
        match &self.last_node {
            Some(node) => {
                let new_node = Rc::new(RefCell::new(ListNode::new(
                    data,
                    Some(Rc::clone(&node)),
                    None,
                )));
                self.last_node = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(ListNode::new(data, None, None)));
                self.first_node = Some(Rc::clone(&new_node));
                self.last_node = Some(new_node);
            }
        };

        self.length += 1;
    }

    fn insert_begin(&mut self, data: T) {
        match &self.first_node {
            Some(node) => {
                let new_node = Rc::new(RefCell::new(ListNode::new(
                    data,
                    None,
                    Some(Rc::clone(&node)),
                )));
                self.first_node = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(ListNode::new(data, None, None)));
                self.first_node = Some(Rc::clone(&new_node));
                self.last_node = Some(new_node);
            }
        }

        self.length += 1;
    }

    fn remove_last(&mut self) -> Option<T> {
        match self.last_node.take() {
            Some(node) => {
                let node = Rc::try_unwrap(node).ok().unwrap().into_inner();
                match node.previous_node {
                    Some(prev_node) => {
                        prev_node.borrow_mut().next_node = None;
                        self.last_node = Some(prev_node);
                    }
                    None => {
                        self.first_node = None;
                    }
                }
                self.length -= 1;
                Some(node.data)
            }
            None => None,
        }
    }

    fn remove_first(&mut self) -> Option<T> {
        match self.first_node.take() {
            Some(node) => {
                let node = Rc::try_unwrap(node).ok().unwrap().into_inner();
                match node.next_node {
                    Some(nx_node) => {
                        nx_node.borrow_mut().previous_node = None;
                        self.first_node = Some(nx_node);
                    }
                    None => {
                        self.last_node = None;
                    }
                }
                self.length -= 1;
                Some(node.data)
            }
            None => None,
        }
    }
}
