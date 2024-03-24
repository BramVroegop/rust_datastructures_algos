struct Node<T> {
    value: T,
    next: Option<*mut Node<T>>,
}

struct LinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, value: T) {
        let ptr = Box::into_raw(Box::new(Node::<T> { value, next: None }));

        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(ptr);
            }
        } else {
            self.head = Some(ptr);
        }
        self.tail = Some(ptr);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head {
            self.head = unsafe { (*head).next };
            let boxed = unsafe { Box::from_raw(head) };
            Some(boxed.value)
        } else {
            None
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr_node = self.head;
        while let Some(node) = curr_node {
            let _boxed = unsafe {
                curr_node = (*node).next;
                Box::from_raw(node)
            };
        }
    }
}
