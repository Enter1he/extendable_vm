use crate::machine::instruction_table::Instruction;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::errors::{RuntimeError, TypeError};
use crate::jex::values::{JexValue, JexObject};
use std::rc::Rc;
use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;

pub const ARITHMETIC_INSTRUCTIONS: Vec<JexInstruction> = vec![
    Instruction {
        op_code: 15,
        name: "NEGATE".to_string(),
        byte_arity: 0,
        instruction_fn: negate_instruction
    },
    Instruction {
        op_code: 16,
        name: "ADD".to_string(),
        byte_arity: 0,
        instruction_fn: add_instruction
    },
    Instruction {
        op_code: 17,
        name: "SUBTRACT".to_string(),
        byte_arity: 0,
        instruction_fn: subtract_instruction
    },
    Instruction {
        op_code: 18,
        name: "MULTIPLY".to_string(),
        byte_arity: 0,
        instruction_fn: multiply_instruction
    },
    Instruction {
        op_code: 19,
        name: "DIVIDE".to_string(),
        byte_arity: 0,
        instruction_fn: divide_instruction
    },
];


fn negate_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let value = machine.stack.pop()?.as_int()?;
    machine.stack.push(JexValue::Int(-value));
    Ok(())
}

fn add_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let result = match (left, right) {
        (JexValue::Int(left), JexValue::Int(right)) => Ok(JexValue::Int(left + right)),
        (JexValue::Object(left), JexValue::Object(right)) => {
            let JexObject::String(left) = &*left;
            let JexObject::String(right) = &*right;
            let result = left.clone() + right;
            Ok(JexValue::Object(Rc::new(JexObject::String(result))))
        }
        _ => {
            let message = format!(
                "ADD not supported for {} and {}",
                left.to_output_string(),
                right.to_output_string()
            );
            Err(TypeError(message))
        }
    }?;
    machine.stack.push(result);
    Ok(())
}

fn subtract_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Int(left - right));
    Ok(())
}

fn multiply_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Int(left * right));
    Ok(())
}

fn divide_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), impl RuntimeError> {
    let (left, right) = machine.stack.pop_two_operands()?;
    let (left, right) = (left.as_int()?, right.as_int()?);
    machine.stack.push(JexValue::Int(left / right));
    Ok(())
}
