use std::usize;

use opcode::arity;
use parameter::Parameter;

pub mod parameter;
pub mod opcode {
    pub const ADD: isize = 1;
    pub const MULT: isize = 2;
    pub const INPUT: isize = 3;
    pub const OUTPUT: isize = 4;
    pub const HALT: isize = 99;

    pub fn arity(opcode: isize) -> usize {
        match opcode {
            ADD => 3,
            MULT => 3,
            INPUT => 1,
            OUTPUT => 1,
            HALT => 0,
            _ => panic!("checking arity of unknown opcode {opcode}"),
        }
    }
}

pub enum Instruction {
    Add(Parameter, Parameter, Parameter),      // 1
    Multiply(Parameter, Parameter, Parameter), // 2
    Input(Parameter),                          // 3
    Output(Parameter),                         // 4
    Halt,                                      // 99
}

impl Instruction {
    fn opcode(&self) -> isize {
        match *self {
            Self::Add(..) => opcode::ADD,
            Self::Multiply(..) => opcode::MULT,
            Self::Input(..) => opcode::INPUT,
            Self::Output(..) => opcode::OUTPUT,
            Self::Halt => opcode::HALT,
        }
    }

    pub fn arity(&self) -> usize {
        arity(self.opcode())
    }
}
