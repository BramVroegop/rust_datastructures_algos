pub trait Stack<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> T;
    fn peek(&self) -> &T;
    fn size(&self) -> usize;
}

pub struct StackStruct<T> {
    values: Vec<T>,
}

impl<T> StackStruct<T> {
    pub fn new() -> StackStruct<T> {
        StackStruct { values: Vec::new() }
    }
}

impl<T> Stack<T> for StackStruct<T> {
    fn size(&self) -> usize {
        self.values.len()
    }

    fn push(&mut self, value: T) {
        self.values.push(value);
    }

    fn pop(&mut self) -> T {
        let value = self.values.pop().unwrap();
        value
    }

    fn peek(&self) -> &T {
        let value = self.values.last().unwrap();
        value
    }
}
