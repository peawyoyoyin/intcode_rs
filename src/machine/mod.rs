use std::collections::VecDeque;

use instruction::{opcode, Instruction};

pub struct IntCodeMachine {
  memory: Vec<usize>,
  pc: usize,
  halted: bool,
  suspended: bool,
  input: VecDeque<usize>,
  output: Vec<usize>,
}

mod instruction;

impl IntCodeMachine {
  pub fn new(initial_memory: Vec<usize>) -> IntCodeMachine {
    IntCodeMachine {
      memory: initial_memory,
      pc: 0,
      halted: false,
      suspended: false,
      input: VecDeque::new(),
      output: Vec::new(),
    }
  }

  pub fn memory(&self, address: usize) -> Option<&usize> {
    self.memory.get(address)
  }

  pub fn halted(&self) -> bool {
    self.halted
  }

  pub fn output(&self) -> &Vec<usize> {
    &self.output
  }

  pub fn feed_input(&mut self, item: usize) {
    self.input.push_back(item);
  }

  fn fetch_and_decode(&self) -> Instruction {
    let opcode = self.memory.get(self.pc).expect(
      &format!("pc is at {} which is outside of available memory", self.pc)
    );
    let arity = opcode::arity(*opcode);
    let parameters = self.memory.get(
      self.pc+1..self.pc+arity+1
    ).expect(
      &format!("error while reading parameters (pc is at {}, arity {arity})", self.pc)
    ); 

    match *opcode {
      opcode::ADD => Instruction::Add(parameters[0], parameters[1], parameters[2]), 
      opcode::MULT => Instruction::Multiply(parameters[0], parameters[1], parameters[2]),
      opcode::INPUT => Instruction::Input(parameters[0]),
      opcode::OUTPUT => Instruction::Output(parameters[0]),
      opcode::HALT => Instruction::Halt,
      _ => panic!("unknown opcode {opcode} while decoding")
    }
  }

  fn execute(&mut self, instruction: &Instruction) {
    let mut should_advance_pc = true;

    match *instruction {
      Instruction::Add(a, b, c) => {
        self.memory[c] = self.memory[a] + self.memory[b];
      }
      Instruction::Multiply(a, b, c) => {
        self.memory[c] = self.memory[a] * self.memory[b];
      }
      Instruction::Input(a) => {
        match self.input.front() {
          Some(item) => {
            self.memory[a] = *item;
            self.input.pop_front();
          }
          None => {
            should_advance_pc = false;
            self.suspended = true;
          }
        }
      }
      Instruction::Output(a) => {
        self.output.push(self.memory[a]);
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
      let instruction = self.fetch_and_decode();
      self.execute(&instruction);
    }
  }

  pub fn run_till_halt(&mut self) {
    while !self.halted {
      if self.suspended {
        panic!("unexpected suspend while running run_till_halt")
      }
      let instruction = self.fetch_and_decode();
      self.execute(&instruction);
    }
  }
}

#[cfg(test)]
mod tests {
    use super::IntCodeMachine;

  #[test]
  fn test_day2_simple_case1() {
    let mut machine = IntCodeMachine::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
    machine.run_till_halt();
    assert!(machine.halted());
    assert_eq!(machine.memory(0), Some(3500).as_ref());
  }

  #[test]
  fn test_day5_simple_case1() {
    let mut machine = IntCodeMachine::new(vec![3,0,4,0,99]);
    machine.run();
    machine.feed_input(17);
    machine.run();
    assert!(machine.halted());
    assert_eq!(machine.output().get(0), Some(17).as_ref());
  }
}
