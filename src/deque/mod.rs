use core::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

struct Node<T: Debug> {
    value: T,
    previous: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Deque<T: Debug> {
    size: u32,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> Deque<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn queue_front(&mut self, value: T) {
        let node = Node::<T> {
            value,
            next: None,
            previous: None,
        };

        let ptr = Rc::new(RefCell::new(node));

        if let Some(head) = self.head.take() {
            head.borrow_mut().previous = Some(ptr.clone());
            ptr.borrow_mut().next = Some(head);
        } else {
            self.tail = Some(ptr.clone());
        }
        self.head = Some(ptr);
        self.size += 1;
    }

    pub fn queue_back(&mut self, value: T) {
        let node = Node::<T> {
            value,
            next: None,
            previous: None,
        };

        let ptr = Rc::new(RefCell::new(node));

        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(ptr.clone());
            ptr.borrow_mut().previous = Some(tail);
        } else {
            self.head = Some(ptr.clone());
        }
        self.tail = Some(ptr);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            if let Some(next) = &mut head.borrow_mut().next {
                next.borrow_mut().previous = None;
            }

            if self.size == 1 {
                self.tail = None;
            }

            if let Some(rfc) = Rc::try_unwrap(head).ok() {
                let node = rfc.into_inner();
                self.head = node.next;
                self.size -= 1;

                Some(node.value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            if let Some(prev) = &mut tail.borrow_mut().previous {
                prev.borrow_mut().next = None;
            }

            if self.size == 1 {
                self.head = None;
            }

            if let Some(rfc) = Rc::try_unwrap(tail).ok() {
                let node = rfc.into_inner();
                self.tail = node.previous;
                self.size -= 1;

                Some(node.value)
            } else {
                None
            }
        } else {
            None
        }
    }
}
