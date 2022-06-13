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
        else {
            *self.locals.get_mut(local_idx) = value;
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
