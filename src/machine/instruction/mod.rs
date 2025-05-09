use std::usize;

use opcode::arity;
use parameter::Parameter;

use crate::machine::Data;

pub mod parameter;
pub mod opcode {
    use crate::machine::Data;

    pub const ADD: Data = 1;
    pub const MULT: Data = 2;
    pub const INPUT: Data = 3;
    pub const OUTPUT: Data = 4;
    pub const JUMP_IF_TRUE: Data = 5;
    pub const JUMP_IF_FALSE: Data = 6;
    pub const LESS_THAN: Data = 7;
    pub const EQUAL: Data = 8;
    pub const HALT: Data = 99;

    pub fn arity(opcode: Data) -> usize {
        match opcode {
            ADD => 3,
            MULT => 3,
            INPUT => 1,
            OUTPUT => 1,
            JUMP_IF_TRUE => 2,
            JUMP_IF_FALSE => 2,
            LESS_THAN => 3,
            EQUAL => 3,
            HALT => 0,
            _ => panic!("checking arity of unknown opcode {opcode}"),
        }
    }
}

pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),         
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter),
    Halt,
}

impl Instruction {
    fn opcode(&self) -> Data {
        match *self {
            Self::Add(..) => opcode::ADD,
            Self::Multiply(..) => opcode::MULT,
            Self::Input(..) => opcode::INPUT,
            Self::Output(..) => opcode::OUTPUT,
            Self::JumpIfTrue(..) => opcode::JUMP_IF_TRUE,
            Self::JumpIfFalse(..) => opcode::JUMP_IF_FALSE,
            Self::LessThan(..) => opcode::LESS_THAN,
            Self::Equal(..) => opcode::EQUAL,
            Self::Halt => opcode::HALT,
        }
    }

    pub fn arity(&self) -> usize {
        arity(self.opcode())
    }
}
