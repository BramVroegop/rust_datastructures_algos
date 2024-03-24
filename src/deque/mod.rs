use std::{cell::RefCell, rc::Rc};

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Deque<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: u32,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn queue_front(&self, value: T) {}

    pub fn queue_back(&self, value: T) {}
}
