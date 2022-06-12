#[derive(Debug)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(Vec::new())
    }

    pub fn get(&self, idx: usize) -> &T {
        self.0.get(idx).expect("Wrong stack index.")
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        self.0.get_mut(idx).expect("Wrong stack index.")
    }

    pub fn push(&mut self, val: T) {
        self.0.push(val);
    }

    pub fn pop(&mut self) -> T {
        self.0.pop().expect("Stack is empty, nothing to pop.")
    }

    pub fn peek(&self) -> &T {
        self.0.last().expect("Stack is empty, nothing to peek.")
    }

    pub fn peek_mut(&mut self) -> &mut T {
        self.0.last_mut().expect("Stack is empty, nothing to peek.")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
