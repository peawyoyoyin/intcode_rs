#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParameterMode {
  Position,
  Immediate
}

impl ParameterMode {
  pub fn from_int(int: isize) -> ParameterMode {
    match int {
      0 => Self::Position,
      1 => Self::Immediate,
      _ => panic!("unknown parameter mode {int}")
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Parameter {
  pub value: isize,
  pub mode: ParameterMode
}

impl Parameter {
  pub fn from(pair: (&isize, ParameterMode)) -> Parameter {
    Parameter { value: *pair.0, mode: pair.1 }
  }
}
