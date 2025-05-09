use crate::machine::Data;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParameterMode {
  Position,
  Immediate,
  Relative
}

impl ParameterMode {
  pub fn from_int(int: Data) -> ParameterMode {
    match int {
      0 => Self::Position,
      1 => Self::Immediate,
      2 => Self::Relative,
      _ => panic!("unknown parameter mode {int}")
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Parameter {
  pub value: Data,
  pub mode: ParameterMode
}

impl Parameter {
  pub fn from(pair: (&Data, ParameterMode)) -> Parameter {
    Parameter { value: *pair.0, mode: pair.1 }
  }
}
