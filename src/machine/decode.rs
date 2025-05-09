use crate::machine::instruction::Instruction;
use crate::machine::instruction::opcode;
use crate::machine::instruction::parameter::{Parameter, ParameterMode};
use crate::utils::log::Logger;

use crate::machine::{Data, Address};

/**
 * returns (opcode, parameter modes)
 *
 */
fn parse_parameter_modes(raw_opcode: Data) -> (isize, Vec<ParameterMode>) {
    let parsed_opcode = raw_opcode % 100;

    let mut parameter_modes: Vec<ParameterMode> = Vec::new();
    let arity = opcode::arity(parsed_opcode);
    let mut mode_indicator = raw_opcode / 100;

    for _ in 0..arity {
        let mode_int = mode_indicator % 10;
        parameter_modes.push(ParameterMode::from_int(mode_int));
        mode_indicator /= 10;
    }

    (parsed_opcode, parameter_modes)
}

pub fn fetch_and_decode(memory: &Vec<Data>, pc: Address) -> Instruction {
    let logger = crate::logger!("fetch_and_decode");
    logger.debug(format!("Start fetch and decode at PC={pc}"));

    let raw_opcode = memory.get(pc).expect(&format!(
        "pc is at {} which is outside of available memory",
        pc
    ));
    logger.debug(format!("raw opcode = {raw_opcode}"));

    let (parsed_opcode, parameter_modes) = parse_parameter_modes(*raw_opcode);
    let arity = opcode::arity(parsed_opcode);
    let parameters_values = memory.get((pc + 1)..(pc + arity + 1)).expect(&format!(
        "error while reading parameters (pc is at {}, arity {arity})",
        pc
    ));

    let parameters: Vec<Parameter> = parameters_values
        .iter()
        .zip(parameter_modes)
        .map(|pair| Parameter::from(pair))
        .collect();

    match parsed_opcode {
        opcode::ADD => Instruction::Add(parameters[0], parameters[1], parameters[2]),
        opcode::MULT => Instruction::Multiply(parameters[0], parameters[1], parameters[2]),
        opcode::INPUT => Instruction::Input(parameters[0]),
        opcode::OUTPUT => Instruction::Output(parameters[0]),
        opcode::JUMP_IF_TRUE => Instruction::JumpIfTrue(parameters[0], parameters[1]),
        opcode::JUMP_IF_FALSE => Instruction::JumpIfFalse(parameters[0], parameters[1]),
        opcode::LESS_THAN => Instruction::LessThan(parameters[0], parameters[1], parameters[2]),
        opcode::EQUAL => Instruction::Equal(parameters[0], parameters[1], parameters[2]),
        opcode::ADJUST_RELATIVE_BASE => Instruction::AdjustRelativeBase(parameters[0]),
        opcode::HALT => Instruction::Halt,
        _ => panic!("unknown opcode {raw_opcode} while decoding"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::instruction::parameter::ParameterMode;

    #[test]
    fn test_parse_parameter_modes() {
        let result = parse_parameter_modes(1002);
        assert_eq!(result.0, 2);
        assert_eq!(
            result.1,
            vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            ]
        )
    }
}
