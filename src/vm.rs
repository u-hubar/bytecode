use crate::{stack::Stack, frame::Frame, instruction::Instruction};

pub type Pointer = usize;

const CALL_STACK_DEFAULT_CAPACITY: usize = 20;

#[derive(Debug, PartialEq)]
pub struct VirtualMachine {
    ip: Pointer,
    call_stack: Stack<Frame<isize>>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            ip: 0,
            call_stack: Stack::with_capacity(CALL_STACK_DEFAULT_CAPACITY),
        }
    }

    pub fn run(&mut self, program: Vec<Instruction>) {
        let main_frame = Frame::new(program.len());
        self.call_stack.push(main_frame);

        while let Some(instruction) = program.get(self.ip) {
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
            .unwrap()
            .push_value(value);
    }

    pub fn pop_value(&mut self) -> isize {
        self.call_stack
            .peek_mut()
            .unwrap()
            .pop_value()
            .unwrap()
    }

    pub fn peek_value(&self) -> &isize {
        self.call_stack
            .peek()
            .unwrap()
            .peek_value()
            .unwrap()
    }

    pub fn write_variable(&mut self, var_idx: usize) {
        let val = self.pop_value();

        self.call_stack
            .peek_mut()
            .unwrap()
            .set_local(var_idx, val);
    }

    pub fn read_variable(&mut self, var_idx: usize) {
        self.push_value(
            *self.call_stack
                .peek()
                .unwrap()
                .get_local(var_idx)
                .unwrap()
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
            .unwrap()
            .get_local(var_idx)
            .unwrap();

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
        self.ip = self.call_stack.pop().unwrap().ip;
    }

    pub fn return_value(&mut self) {
        let val = self.pop_value();
        self.ip = self.call_stack.pop().unwrap().ip;
        self.push_value(val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let vm = VirtualMachine::new();

        assert_eq!(vm.ip, 0);
        assert!(vm.call_stack.is_empty());
    }

    #[test]
    fn run() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        let program = vec![
            Instruction::LoadValue(1),      // LOAD_VAL 1
            Instruction::WriteVariable(0),  // WRITE_VAR 'x'
            Instruction::Ignore,            // LABEL LOOP
            Instruction::ReadVariable(0),   // READ_VAR 'x'
            Instruction::CallFunction(11),  // CALL TEST
            Instruction::Add,               // ADD
            Instruction::WriteVariable(0),  // WRITE_VAR 'x'
            Instruction::ReadVariable(0),   // READ_VAR 'x'
            Instruction::LoadValue(10),     // LOAD_VAL 10
            Instruction::JumpIfSmaller(2),  // JUMP_IF_SM LOOP
            Instruction::ReadVariable(0),   // READ_VAR 'x'
            Instruction::Jump(17),          // FUNC TEST
            Instruction::LoadValue(5),      // LOAD_VAL 5
            Instruction::WriteVariable(0),  // WRITE_VAR 'x'
            Instruction::ReadVariable(0),   // READ_VAR 'x'
            Instruction::LoadValue(5),      // LOAD_VAL 5
            Instruction::Divide,            // DIVIDE
            Instruction::ReturnValue,       // RETURN_VAL
        ];

        vm.run(program);

        let actual_frame = vm.call_stack.peek_mut().unwrap();

        assert_eq!(actual_frame.pop_value().unwrap(), 10);
    }

    #[test]
    fn push_value() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);

        let mut expected_call_stack = Stack::new();
        let mut expected_frame = Frame::new(0);
        expected_frame.get_operand_stack_mut().push(10);
        expected_call_stack.push(expected_frame);

        assert_eq!(vm.call_stack, expected_call_stack);
    }

    #[test]
    fn pop_value() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        let val = vm.pop_value();

        assert_eq!(val, 10);
    }

    #[test]
    fn peek_value() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        let val = vm.peek_value();

        assert_eq!(val, &10);
    }

    #[test]
    fn write_variable() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.write_variable(0);

        let actual_frame = vm.call_stack.peek().unwrap();

        assert_eq!(actual_frame.get_local(0).unwrap(), &10);
    }

    #[test]
    fn read_variable() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.write_variable(0);
        vm.read_variable(0);

        let actual_frame = vm.call_stack.peek_mut().unwrap();

        assert_eq!(actual_frame.pop_value().unwrap(), 10);
    }

    #[test]
    fn add() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(5);
        vm.push_value(10);
        vm.add();

        assert_eq!(vm.pop_value(), 15);
    }

    #[test]
    fn sub() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.push_value(3);
        vm.sub();

        assert_eq!(vm.pop_value(), 7);
    }

    #[test]
    fn multiply() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(5);
        vm.push_value(10);
        vm.multiply();

        assert_eq!(vm.pop_value(), 50);
    }

    #[test]
    fn divide() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.push_value(5);
        vm.divide();

        assert_eq!(vm.pop_value(), 2);
    }

    #[test]
    fn jump() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.jump(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn jie() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(5);
        vm.push_value(5);
        vm.jie(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn jine() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.push_value(5);
        vm.jine(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn jilg() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.push_value(5);
        vm.jilg(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn jils() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(5);
        vm.push_value(10);
        vm.jils(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn jilge() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.push_value(5);
        vm.jilge(5);

        assert_eq!(vm.ip, 5);

        vm.push_value(5);
        vm.push_value(5);
        vm.jilge(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn jilse() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);

        vm.push_value(5);
        vm.push_value(10);
        vm.jilse(5);

        assert_eq!(vm.ip, 5);

        vm.push_value(5);
        vm.push_value(5);
        vm.jilse(10);

        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn call_function() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(5);
        vm.call_stack.push(frame);
        vm.ip = 5;

        vm.call_function(10);

        let actual_frame = vm.call_stack.peek().unwrap();

        assert_eq!(actual_frame.ip, 5);
        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn return_void() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);
        let frame: Frame<isize> = Frame::new(5);
        vm.call_stack.push(frame);

        vm.return_void();

        assert_eq!(vm.ip, 5);
    }

    #[test]
    fn return_value() {
        let mut vm = VirtualMachine::new();
        let frame: Frame<isize> = Frame::new(0);
        vm.call_stack.push(frame);
        let frame: Frame<isize> = Frame::new(5);
        vm.call_stack.push(frame);

        vm.push_value(10);
        vm.return_value();

        let actual_frame = vm.call_stack.peek_mut().unwrap();

        assert_eq!(actual_frame.pop_value().unwrap(), 10);
        assert_eq!(vm.ip, 5);
    }
}