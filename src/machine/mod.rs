use std::collections::VecDeque;

use decode::fetch_and_decode;
use instruction::Instruction;
use instruction::parameter::{Parameter, ParameterMode};

mod decode;
mod instruction;

pub type Data = isize;
pub type Address = usize;

pub struct IntCodeMachine {
    memory: Vec<Data>,
    pc: Address,
    halted: bool,
    suspended: bool,
    relative_base: Data,
    input: VecDeque<Data>,
    output: Vec<Data>,
}

impl IntCodeMachine {
    pub fn new(initial_memory: Vec<Data>) -> IntCodeMachine {
        IntCodeMachine {
            memory: initial_memory,
            pc: 0,
            relative_base: 0,
            halted: false,
            suspended: false,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }

    pub fn memory(&self, address: Address) -> Option<&Data> {
        self.memory.get(address)
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn output(&self) -> &Vec<Data> {
        &self.output
    }

    pub fn feed_input(&mut self, item: Data) {
        self.input.push_back(item);
    }

    fn resolve_parameter(&self, parameter: Parameter) -> Data {
        match parameter.mode {
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Position => {
                assert!(
                    parameter.value >= 0,
                    "unexpected negative value {} for position mode parameter",
                    parameter.value
                );
                self.memory[parameter.value.unsigned_abs()]
            }
            ParameterMode::Relative => {
                let resolved_address = parameter.value + self.relative_base;
                assert!(
                    resolved_address >= 0,
                    "relative mode parameter resolves to a negative address {}, parameter value= {}, relative base = {}",
                    resolved_address,
                    parameter.value,
                    self.relative_base
                );
                self.memory[resolved_address.unsigned_abs()]
            }
        }
    }

    fn resolve_address(&self, parameter: Parameter) -> Address {
        assert!(
            parameter.mode != ParameterMode::Immediate,
            "unexpected immediate mode for address parameter {}",
            parameter.value
        );
        assert!(
            parameter.value >= 0,
            "unexpected negative value {} for address parameter",
            parameter.value
        );
        parameter.value.unsigned_abs()
    }

    fn execute(&mut self, instruction: &Instruction) {
        let mut should_advance_pc = true;

        match *instruction {
            Instruction::Add(a, b, c) => {
                let address = self.resolve_address(c);
                self.memory[address] = self.resolve_parameter(a) + self.resolve_parameter(b);
            }
            Instruction::Multiply(a, b, c) => {
                let address = self.resolve_address(c);
                self.memory[address] = self.resolve_parameter(a) * self.resolve_parameter(b);
            }
            Instruction::Input(a) => match self.input.front() {
                Some(item) => {
                    let address = self.resolve_address(a);
                    self.memory[address] = *item;
                    self.input.pop_front();
                }
                None => {
                    should_advance_pc = false;
                    self.suspended = true;
                }
            },
            Instruction::Output(a) => {
                self.output.push(self.resolve_parameter(a));
            }
            Instruction::JumpIfTrue(a, b) => {
                if self.resolve_parameter(a) != 0 {
                    self.pc = self.resolve_parameter(b).unsigned_abs();
                    should_advance_pc = false;
                }
            }
            Instruction::JumpIfFalse(a, b) => {
                if self.resolve_parameter(a) == 0 {
                    self.pc = self.resolve_parameter(b).unsigned_abs();
                    should_advance_pc = false;
                }
            }
            Instruction::LessThan(a, b, c) => {
                let address = self.resolve_address(c);
                self.memory[address] = if self.resolve_parameter(a) < self.resolve_parameter(b) {
                    1
                } else {
                    0
                }
            }
            Instruction::Equal(a, b, c) => {
                let address = self.resolve_address(c);
                self.memory[address] = if self.resolve_parameter(a) == self.resolve_parameter(b) {
                    1
                } else {
                    0
                }
            }
            Instruction::AdjustRelativeBase(a) => {
                self.relative_base += self.resolve_parameter(a);
            }
            Instruction::Halt => {
                self.halted = true;
            }
        }

        if should_advance_pc {
            self.pc += instruction.arity() + 1;
        }
    }

    pub fn run(&mut self) {
        self.suspended = false;
        while !(self.suspended || self.halted) {
            let instruction = fetch_and_decode(&self.memory, self.pc);
            self.execute(&instruction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_simple_case1() {
        let mut machine = IntCodeMachine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        machine.run();
        assert!(machine.halted());
        assert_eq!(machine.memory(0), Some(3500).as_ref());
    }

    #[test]
    fn test_day5_simple_case1() {
        let mut machine = IntCodeMachine::new(vec![3, 0, 4, 0, 99]);
        machine.run();
        machine.feed_input(17);
        machine.run();
        assert!(machine.halted());
        assert_eq!(machine.output().get(0), Some(17).as_ref());
    }

    #[test]
    fn test_day5_jump_case1() {
        let mut machine = IntCodeMachine::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        machine.run();
        machine.feed_input(15);
        machine.run();
        assert!(machine.halted());
        assert_eq!(machine.output().get(0), Some(1).as_ref());
    }

    #[test]
    fn test_day5_jump_case2() {
        let mut machine = IntCodeMachine::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        machine.run();
        machine.feed_input(0);
        machine.run();
        assert!(machine.halted());
        assert_eq!(machine.output().get(0), Some(0).as_ref());
    }

    #[test]
    fn test_day5_jump_case3() {
        let mut machine =
            IntCodeMachine::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        machine.run();
        machine.feed_input(0);
        machine.run();
        assert!(machine.halted());
        assert_eq!(machine.output().get(0), Some(0).as_ref());
    }

    #[test]
    fn test_day5_jump_case4() {
        let mut machine =
            IntCodeMachine::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        machine.run();
        machine.feed_input(9);
        machine.run();
        assert!(machine.halted());
        assert_eq!(machine.output().get(0), Some(1).as_ref());
    }
}
