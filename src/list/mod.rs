use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct ListNode<T> {
    data: T,
    next_node: Option<Rc<RefCell<ListNode<T>>>>,
    previous_node: Option<Weak<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode {
            data,
            next_node: None,
            previous_node: None,
        }))
    }
}

pub struct List<T> {
    first_node: Option<Rc<RefCell<ListNode<T>>>>,
    last_node: Option<Weak<RefCell<ListNode<T>>>>,
    length: usize,
}

impl<T: Debug> List<T> {
    pub fn get_first_node(&self) -> Option<Rc<RefCell<ListNode<T>>>> {
        match &self.first_node {
            Some(node) => Some(Rc::clone(node)),
            None => None,
        }
    }

    pub fn get_last_node(&self) -> Option<Weak<RefCell<ListNode<T>>>> {
        match &self.last_node {
            Some(node) => Some(Weak::clone(node)),
            None => None,
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            first_node: None,
            last_node: None,
            length: 0,
        }
    }

    pub fn insert_first(&mut self, data: T) {
        let new_node = ListNode::new(data);
        match self.first_node.take() {
            Some(old_first_node) => {
                old_first_node.borrow_mut().previous_node = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next_node = Some(old_first_node);
            }
            None => self.last_node = Some(Rc::downgrade(&new_node)),
        };
        self.length += 1;
        self.first_node = Some(new_node);
    }

    pub fn insert_last(&mut self, data: T) {
        let new_node = ListNode::new(data);
        match self.last_node.take() {
            Some(old_last_node) => {
                old_last_node.upgrade().map(|old_last_node_rc| {
                    old_last_node_rc.borrow_mut().next_node = Some(new_node.clone());
                    new_node.borrow_mut().previous_node = Some(old_last_node);
                });
            }
            None => self.first_node = Some(new_node.clone()),
        };
        self.length += 1;
        self.last_node = Some(Rc::downgrade(&new_node));
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.first_node.take().map(|old_first_node| {
            if let Some(new_first_node) = old_first_node.borrow_mut().next_node.take() {
                new_first_node.borrow_mut().previous_node = None;
                self.first_node = Some(new_first_node);
            } else {
                self.last_node.take();
            }
            self.length -= 1;
            Rc::try_unwrap(old_first_node)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }

    pub fn remove_last(&mut self) -> Option<T> {
        self.last_node.take().and_then(|old_last_node| {
            old_last_node.upgrade().map(|old_last_node_rc| {
                if let Some(new_last_node) = old_last_node_rc.borrow_mut().previous_node.take() {
                    new_last_node.upgrade().map(|new_last_node_rc| {
                        new_last_node_rc.borrow_mut().next_node = None;
                        self.last_node = Some(new_last_node);
                    });
                } else {
                    self.first_node.take();
                }
                self.length -= 1;
                Rc::try_unwrap(old_last_node_rc)
                    .ok()
                    .unwrap()
                    .into_inner()
                    .data
            })
        })
    }
}
