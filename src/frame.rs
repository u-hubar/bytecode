use crate::{vm::Pointer, stack::Stack};

#[derive(Debug)]
pub struct Frame<T> {
    pub ip: Pointer,
    operand_stack: Stack<T>,
    locals: Stack<T>,
}

impl<T: Copy> Frame<T> {
    pub fn new(ip: Pointer) -> Self {
        Self {
            ip,
            operand_stack: Stack::new(),
            locals: Stack::new(),
        }
    }

    pub fn push_value(&mut self, value: T) {
        self.operand_stack.push(value);
    }

    pub fn pop_value(&mut self) -> T {
        self.operand_stack.pop()
    }

    pub fn peek_value(&self) -> &T {
        self.operand_stack.peek()
    }

    pub fn set_local(&mut self, local_idx: usize, value: T) {
        if local_idx == self.locals.len() {
            self.locals.push(value);
        }
        else if local_idx < self.locals.len() {
            *self.locals.get_mut(local_idx) = value;
        }
        else {
            panic!("Invalid local variable address.")
        }
    }

    pub fn get_local(&self, local_idx: usize) -> T {
        *self.locals.get(local_idx)
    }

    pub fn get_operand_stack(&self) -> &Stack<T> {
        &self.operand_stack
    }

    pub fn get_operand_stack_mut(&mut self) -> &mut Stack<T> {
        &mut self.operand_stack
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let frame: Frame<isize> = Frame::new(5);

        assert_eq!(frame.ip, 5);
        assert!(frame.operand_stack.is_empty());
        assert!(frame.locals.is_empty());
    }

    #[test]
    fn push_value() {
        let mut frame: Frame<isize> = Frame::new(5);

        frame.push_value(10);

        assert!(!frame.operand_stack.is_empty());
    }

    #[test]
    fn pop_value() {
        let mut frame: Frame<isize> = Frame::new(5);

        frame.push_value(10);

        assert_eq!(frame.pop_value(), 10);
    }

    #[test]
    fn peek_value() {
        let mut frame: Frame<isize> = Frame::new(5);

        frame.push_value(10);

        assert_eq!(*frame.peek_value(), 10);
    }

    #[test]
    fn set_get_local() {
        let mut frame: Frame<isize> = Frame::new(5);

        frame.set_local(0, 10);

        assert_eq!(frame.get_local(0), 10);
    }

    #[test]
    #[should_panic(expected = "Invalid local variable address.")]
    fn set_local_wrong_index() {
        let mut frame: Frame<isize> = Frame::new(5);

        frame.set_local(10, 10);
    }

    #[test]
    fn get_operand_stack() {
        let frame: Frame<isize> = Frame::new(5);

        let operand_stack = frame.get_operand_stack();

        assert!(operand_stack.is_empty());
    }

    #[test]
    fn get_operand_stack_mut() {
        let mut frame: Frame<isize> = Frame::new(5);

        let operand_stack = frame.get_operand_stack_mut();

        operand_stack.push(10);

        assert_eq!(operand_stack.pop(), 10);
    }
}
