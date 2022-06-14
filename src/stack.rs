#[derive(Debug)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(Vec::new())
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

    pub fn get(&self, idx: usize) -> &T {
        self.0.get(idx).expect("Wrong stack index.")
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        self.0.get_mut(idx).expect("Wrong stack index.")
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
    fn push() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);

        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);

        assert_eq!(stack.pop(), 10);
    }

    #[test]
    #[should_panic(expected = "Stack is empty, nothing to pop.")]
    fn empty_pop() {
        let mut stack: Stack<isize> = Stack::new();

        stack.pop();
    }

    #[test]
    fn peek() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);

        assert_eq!(*stack.peek(), 10)
    }

    #[test]
    #[should_panic(expected = "Stack is empty, nothing to peek.")]
    fn empty_peek() {
        let stack: Stack<isize> = Stack::new();

        stack.peek();
    }

    #[test]
    fn peek_mut() {
        let mut stack: Stack<isize> = Stack::new();

        stack.push(10);
        *stack.peek_mut() += 5;

        assert_eq!(*stack.peek_mut(), 15);
    }

    #[test]
    #[should_panic(expected = "Stack is empty, nothing to peek.")]
    fn empty_peek_mut() {
        let mut stack: Stack<isize> = Stack::new();

        stack.peek_mut();
    }

    #[test]
    fn get() {
        let mut stack: Stack<isize> = Stack::new();

        for i in 0..3 {
            stack.push(i);
        }

        assert_eq!(*stack.get(1), 1);
    }

    #[test]
    #[should_panic(expected = "Wrong stack index.")]
    fn get_wrong_index() {
        let stack: Stack<isize> = Stack::new();

        stack.get(10);
    }

    #[test]
    fn get_mut() {
        let mut stack: Stack<isize> = Stack::new();

        for i in 0..3 {
            stack.push(i);
        }
        *stack.get_mut(1) = 10;

        assert_eq!(*stack.get_mut(1), 10);
    }

    #[test]
    #[should_panic(expected = "Wrong stack index.")]
    fn get_mut_wrong_index() {
        let mut stack: Stack<isize> = Stack::new();

        stack.get_mut(10);
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

