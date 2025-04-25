use opcode::arity;

pub mod opcode {
  pub const ADD: usize = 1;
  pub const MULT: usize = 2;
  pub const HALT: usize = 99;
  
  pub fn arity(opcode: usize) -> usize {
    match opcode {
      ADD => 3,
      MULT => 3,
      HALT => 0,
      _ => panic!("checking arity of unknown opcode {opcode}")
    }
  }
}

pub enum Instruction {
  Add(usize, usize, usize), // 1
  Multiply(usize, usize, usize), // 2
  Halt // 99
}

impl Instruction {
  fn opcode(&self) -> usize {
    match *self {
      Self::Add(..) => opcode::ADD,
      Self::Multiply(..) => opcode::MULT,
      Self::Halt => opcode::HALT
    }
  }

  pub fn arity(&self) -> usize {
    arity(self.opcode())
  }
}
