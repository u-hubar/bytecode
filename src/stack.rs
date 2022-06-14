use crate::errors::RuntimeError;

#[derive(Debug)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Stack<T> {
        Stack(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, val: T) {
        self.0.push(val);
    }

    pub fn pop(&mut self) -> Result<T, RuntimeError> {
        self.0.pop().ok_or(RuntimeError::EmptyStack)
    }

    pub fn peek(&self) -> Result<&T, RuntimeError> {
        self.0.last().ok_or(RuntimeError::EmptyStack)
    }

    pub fn peek_mut(&mut self) -> Result<&mut T, RuntimeError> {
        self.0.last_mut().ok_or(RuntimeError::EmptyStack)
    }

    pub fn get(&self, idx: usize) -> Result<&T, RuntimeError> {
        self.0.get(idx).ok_or(RuntimeError::WrongStackIndex)
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut T, RuntimeError> {
        self.0.get_mut(idx).ok_or(RuntimeError::WrongStackIndex)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let stack: Stack<isize> = Stack::new();

        assert!(stack.is_empty());
    }

    #[test]
    fn with_capacity() {
        let capacity = 1000;
        let stack: Stack<isize> = Stack::with_capacity(capacity);

        assert_eq!(stack.0.capacity(), capacity);
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);

        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);

        assert_eq!(stack.pop().unwrap(), 10);
    }

    #[test]
    fn pop_should_return_error_when_stack_is_empty() {
        let mut stack: Stack<isize> = Stack::new();

        assert!(stack.pop().is_err());
    }

    #[test]
    fn peek() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);

        assert_eq!(*stack.peek().unwrap(), 10)
    }

    #[test]
    fn peek_should_return_error_when_stack_is_empty() {
        let stack: Stack<isize> = Stack::new();

        assert!(stack.peek().is_err());
    }

    #[test]
    fn peek_mut() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);
        *stack.peek_mut().unwrap() += 5;

        assert_eq!(*stack.peek_mut().unwrap(), 15);
    }

    #[test]
    fn peek_mut_should_return_error_when_stack_is_empty() {
        let mut stack: Stack<isize> = Stack::new();

        assert!(stack.peek_mut().is_err());
    }

    #[test]
    fn get() {
        let mut stack: Stack<isize> = Stack::new();

        for i in 0..3 {
            stack.push(i);
        }

        assert_eq!(*stack.get(1).unwrap(), 1);
    }

    #[test]
    fn get_should_return_error_when_wrong_index() {
        let stack: Stack<isize> = Stack::new();

        assert!(stack.get(10).is_err());
    }

    #[test]
    fn get_mut() {
        let mut stack: Stack<isize> = Stack::new();

        for i in 0..3 {
            stack.push(i);
        }
        *stack.get_mut(1).unwrap() = 10;

        assert_eq!(*stack.get_mut(1).unwrap(), 10);
    }

    #[test]
    fn get_mut_should_return_error_when_wrong_index() {
        let mut stack: Stack<isize> = Stack::new();

        assert!(stack.get_mut(10).is_err());
    }

    #[test]
    fn len() {
        let mut stack: Stack<isize> = Stack::new();

        for i in 0..3 {
            stack.push(i);
        }

        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn is_empty() {
        let mut stack: Stack<isize> = Stack::new();
        assert!(stack.is_empty());

        stack.push(10);
        assert!(!stack.is_empty());
    }
}

