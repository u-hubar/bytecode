use crate::{stack::Stack, frame::Frame, instruction::Instruction};

pub type Pointer = usize;

pub struct VM {
    operand_stack: Stack<isize>,
    call_stack: Stack<Frame<isize>>,
}

impl VM {
    pub fn new() -> Self {
        let frame = Frame::new();
        let mut call_stack = Stack::new();
        call_stack.push(frame);

        Self {
            operand_stack: Stack::new(),
            call_stack
        }
    }

    pub fn run(&mut self, code: Vec<Instruction>) {
        for instruction in code {
            match instruction {
                Instruction::LoadValue(val) => self.push_operand(val),
                Instruction::WriteVariable(var) => self.set_variable(var),
                Instruction::ReadVariable(var) => self.get_variable(var),
                Instruction::Add => self.add(),
                Instruction::Sub => self.sub(),
                Instruction::Multiply => self.multiply(),
                Instruction::Divide => self.divide(),
                Instruction::ReturnValue => self.return_value(),
            }
        }
    }

    pub fn push_operand(&mut self, val: isize) {
        self.operand_stack.push(val);
    }

    pub fn pop_operand(&mut self) -> isize {
        self.operand_stack.pop()
    }

    pub fn add(&mut self) {
        let (rhs, lhs) = (self.operand_stack.pop(), self.operand_stack.pop());
        self.operand_stack.push(lhs + rhs);
    }

    pub fn sub(&mut self) {
        let (rhs, lhs) = (self.operand_stack.pop(), self.operand_stack.pop());
        self.operand_stack.push(lhs - rhs);
    }

    pub fn multiply(&mut self) {
        let (rhs, lhs) = (self.operand_stack.pop(), self.operand_stack.pop());
        self.operand_stack.push(lhs * rhs);
    }

    pub fn divide(&mut self) {
        let (rhs, lhs) = (self.operand_stack.pop(), self.operand_stack.pop());
        self.operand_stack.push(lhs / rhs);
    }

    pub fn set_variable(&mut self, key: String) {
        self.call_stack.peek_mut().insert(key, self.operand_stack.pop());
    }

    pub fn get_variable(&mut self, key: String) {
        self.operand_stack.push(self.call_stack.peek().get(key));
    }

    pub fn return_value(&mut self) {
        self.call_stack.pop();
    }
}
