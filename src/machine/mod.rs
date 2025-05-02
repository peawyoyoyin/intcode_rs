use std::collections::VecDeque;

use decode::fetch_and_decode;
use instruction::Instruction;
use instruction::parameter::{Parameter, ParameterMode};

mod decode;
mod instruction;

pub struct IntCodeMachine {
    memory: Vec<isize>,
    pc: usize,
    halted: bool,
    suspended: bool,
    input: VecDeque<isize>,
    output: Vec<isize>,
}

impl IntCodeMachine {
    pub fn new(initial_memory: Vec<isize>) -> IntCodeMachine {
        IntCodeMachine {
            memory: initial_memory,
            pc: 0,
            halted: false,
            suspended: false,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }

    pub fn memory(&self, address: usize) -> Option<&isize> {
        self.memory.get(address)
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn output(&self) -> &Vec<isize> {
        &self.output
    }

    pub fn feed_input(&mut self, item: isize) {
        self.input.push_back(item);
    }

    fn resolve_parameter(&self, parameter: Parameter) -> isize {
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
        }
    }

    fn resolve_address(&self, parameter: Parameter) -> usize {
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
}
