use crate::{stack::Stack, frame::Frame, instruction::Instruction, labels::Labels};

pub type Pointer = usize;

pub struct VirtualMachine {
    operand_stack: Stack<isize>,
    call_stack: Stack<Frame<isize>>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let frame = Frame::new();
        let mut call_stack = Stack::new();
        call_stack.push(frame);

        Self {
            operand_stack: Stack::new(),
            call_stack,
        }
    }

    pub fn run(&mut self, program: Vec<Instruction>, labels: &mut Labels) {
        let mut ip: Pointer = 0;

        while let Some(instruction) = program.get(ip) {
            match instruction {
                Instruction::LoadValue(val) => self.operand_stack.push(*val),
                Instruction::WriteVariable(var) => self.write_variable(var),
                Instruction::ReadVariable(var) => self.read_variable(var),
                Instruction::Add => self.add(),
                Instruction::Sub => self.sub(),
                Instruction::Multiply => self.multiply(),
                Instruction::Divide => self.divide(),
                Instruction::Print => self.print(),
                Instruction::PrintVariable(var) => self.print_variable(var),
                Instruction::Label => {},
                Instruction::JumpIfEqual(label_key) => {
                    if self.jie() {
                        ip = *labels.get(label_key);
                    }
                },
                Instruction::JumpIfNotEqual(label_key) => {
                    if self.jine() {
                        ip = *labels.get(label_key);
                    }
                },
                Instruction::JumpIfGreater(label_key) => {
                    if self.jilg() {
                        ip = *labels.get(label_key);
                    }
                },
                Instruction::JumpIfSmaller(label_key) => {
                    if self.jils() {
                        ip = *labels.get(label_key);
                    }
                },
                Instruction::JumpIfGreaterEqual(label_key) => {
                    if self.jilge() {
                        ip = *labels.get(label_key);
                    }
                },
                Instruction::JumpIfSmallerEqual(label_key) => {
                    if self.jilse() {
                        ip = *labels.get(label_key);
                    }
                },
                Instruction::ReturnValue => self.return_value(),
            }

            ip += 1;
        }
    }

    pub fn write_variable(&mut self, key: &str) {
        self.call_stack
            .peek_mut()
            .insert(key.to_string(), self.operand_stack.pop());
    }

    pub fn read_variable(&mut self, key: &str) {
        self.operand_stack.push(
            self.call_stack
                .peek()
                .get(key.to_string())
        );
    }

    pub fn add(&mut self) {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );
        self.operand_stack.push(lhs + rhs);
    }

    pub fn sub(&mut self) {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );
        self.operand_stack.push(lhs - rhs);
    }

    pub fn multiply(&mut self) {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );
        self.operand_stack.push(lhs * rhs);
    }

    pub fn divide(&mut self) {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );
        self.operand_stack.push(lhs / rhs);
    }

    pub fn print(&mut self) {
        println!("{}", self.operand_stack.peek());
    }

    pub fn print_variable(&mut self, key: &str) {
        let val = self.call_stack
            .peek()
            .get(key.to_string());

        println!("{}={}", key, val);
    }

    pub fn jie(&mut self) -> bool {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );

        lhs == rhs
    }

    pub fn jine(&mut self) -> bool {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );

        lhs != rhs
    }

    pub fn jilg(&mut self) -> bool {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );

        lhs > rhs
    }

    pub fn jils(&mut self) -> bool {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );

        lhs < rhs
    }

    pub fn jilge(&mut self) -> bool {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );

        lhs >= rhs
    }

    pub fn jilse(&mut self) -> bool {
        let (rhs, lhs) = (
            self.operand_stack.pop(),
            self.operand_stack.pop()
        );

        lhs <= rhs
    }

    pub fn return_value(&mut self) {
        self.call_stack.pop();
    }
}
