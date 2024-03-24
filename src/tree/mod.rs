use std::fmt::Display;

pub trait Comparable {
    fn cmp(&self, other: &Self) -> i8;
}

struct Node<T: Comparable + Display> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct Tree<T: Comparable + Display> {
    root: Option<Box<Node<T>>>,
}

impl<T: Comparable + Display> Tree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn add(&mut self, value: T) {
        let node = Some(Box::new(Node::<T> {
            value,
            left: None,
            right: None,
        }));
        let root_ref = &mut self.root;
        Self::_add(node, root_ref);
    }

    fn _add(node: Option<Box<Node<T>>>, root: &mut Option<Box<Node<T>>>) {
        let root_ref = root.as_mut().unwrap();

        let eq = node.as_ref().unwrap().value.cmp(&root_ref.value);

        if eq > 0 {
            if root_ref.right.is_none() {
                root_ref.right = node;
                return;
            }
            Self::_add(node, &mut root_ref.right);
        } else {
            if root_ref.left.is_none() {
                root_ref.left = node;
                return;
            }
            Self::_add(node, &mut root_ref.left);
        }
    }

    pub fn print(&self) {
        self._print(&self.root);
    }

    fn _print(&self, node: &Option<Box<Node<T>>>) {
        match node {
            Some(e) => {
                println!("{}", e.value);
                self._print(&e.left);
                self._print(&e.right);
            }
            None => {
                return;
            }
        }
    }
}
