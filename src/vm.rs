use crate::{stack::Stack, frame::Frame, instruction::Instruction};

pub type Pointer = usize;

pub struct VirtualMachine {
    ip: Pointer,
    call_stack: Stack<Frame<isize>>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let ip = 0;
        let frame = Frame::new(ip);
        let mut call_stack = Stack::new();
        call_stack.push(frame);

        Self {
            ip,
            call_stack,
        }
    }

    pub fn run(&mut self, program: Vec<Instruction>) {
        while let Some(instruction) = program.get(self.ip) {
            println!("{:?}", instruction);
            match instruction {
                Instruction::LoadValue(val) => self.push_value(*val),
                Instruction::WriteVariable(var_idx) => self.write_variable(*var_idx),
                Instruction::ReadVariable(var_idx) => self.read_variable(*var_idx),
                Instruction::Add => self.add(),
                Instruction::Sub => self.sub(),
                Instruction::Multiply => self.multiply(),
                Instruction::Divide => self.divide(),
                Instruction::Print => self.print(),
                Instruction::PrintVariable(var_name, var_idx) => self.print_variable(var_name, *var_idx),
                Instruction::CallFunction(func_ip) => self.call_function(*func_ip),
                Instruction::Jump(ip) => self.jump(*ip),
                Instruction::JumpIfEqual(label_ip) => self.jie(*label_ip),
                Instruction::JumpIfNotEqual(label_ip) => self.jine(*label_ip),
                Instruction::JumpIfGreater(label_ip) => self.jilg(*label_ip),
                Instruction::JumpIfSmaller(label_ip) => self.jils(*label_ip),
                Instruction::JumpIfGreaterEqual(label_ip) => self.jilge(*label_ip),
                Instruction::JumpIfSmallerEqual(label_ip) => self.jilse(*label_ip),
                Instruction::Return => self.return_void(),
                Instruction::ReturnValue => self.return_value(),
                Instruction::Ignore => {},
            }

            self.ip += 1;
        }
    }

    pub fn push_value(&mut self, value: isize) {
        self.call_stack
            .peek_mut()
            .push_value(value);
    }

    pub fn pop_value(&mut self) -> isize {
        self.call_stack
            .peek_mut()
            .pop_value()
    }

    pub fn peek_value(&self) -> &isize {
        self.call_stack
            .peek()
            .peek_value()
    }

    pub fn write_variable(&mut self, var_idx: usize) {
        let val = self.pop_value();

        self.call_stack
            .peek_mut()
            .set_local(var_idx, val);
    }

    pub fn read_variable(&mut self, var_idx: usize) {
        self.push_value(
            self.call_stack
                .peek()
                .get_local(var_idx)
        );
    }

    pub fn add(&mut self) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );
        self.push_value(lhs + rhs);
    }

    pub fn sub(&mut self) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );
        self.push_value(lhs - rhs);
    }

    pub fn multiply(&mut self) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );
        self.push_value(lhs * rhs);
    }

    pub fn divide(&mut self) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );
        self.push_value(lhs / rhs);
    }

    pub fn print(&self) {
        println!("{}", self.peek_value());
    }

    pub fn print_variable(&self, var_name: &str, var_idx: usize) {
        let val = self.call_stack
            .peek()
            .get_local(var_idx);

        println!("{} = {}", var_name, val);
    }

    pub fn jump(&mut self, ip: Pointer) {
        self.ip = ip;
    }

    pub fn jie(&mut self, label_ip: Pointer) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );

        if lhs == rhs {
            self.ip = label_ip;
        }
    }

    pub fn jine(&mut self, label_ip: Pointer) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );

        if lhs != rhs {
            self.ip = label_ip;
        }
    }

    pub fn jilg(&mut self, label_ip: Pointer) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );

        if lhs > rhs {
            self.ip = label_ip;
        }
    }

    pub fn jils(&mut self, label_ip: Pointer) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );

        if lhs < rhs {
            self.ip = label_ip;
        }
    }

    pub fn jilge(&mut self, label_ip: Pointer) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );

        if lhs >= rhs {
            self.ip = label_ip;
        }
    }

    pub fn jilse(&mut self, label_ip: Pointer) {
        let (rhs, lhs) = (
            self.pop_value(),
            self.pop_value(),
        );

        if lhs <= rhs {
            self.ip = label_ip;
        }
    }

    pub fn call_function(&mut self, start_ip: Pointer) {
        self.call_stack.push(
            Frame::new(
                self.ip,
            )
        );

        self.ip = start_ip;
    }

    pub fn return_void(&mut self) {
        self.ip = self.call_stack.pop().ip;
    }

    pub fn return_value(&mut self) {
        if self.call_stack.len() == 1 {
            self.ip = self.call_stack.pop().ip;
        }
        else {
            let val = self.pop_value();
            self.ip = self.call_stack.pop().ip;
            self.push_value(val);
        }
    }
}
