pub mod code;

pub mod run_jex {
    use crate::run::code::{TestChunk, TestInstruction};
    use extendable_vm::jex::build_jex_machine;
    use extendable_vm::jex::bytecode_constants::JexConstant;
    use extendable_vm::jex::instructions::types::JexInstructionTable;
    use extendable_vm::jex::instructions::JEX_INSTRUCTIONS;
    use extendable_vm::jex::jex_values::values::{JexFunction, JexValue};
    use extendable_vm::jex::types::JexMachine;
    use extendable_vm::machine::code::{Chunk, Code};
    use extendable_vm::machine::instruction_table::InstructionTable;
    use extendable_vm::machine::machine::Machine;

    pub fn run_chunks(chunks: Vec<TestChunk>) -> Option<JexValue> {
        let mut compiled_chunks: Vec<Chunk<JexConstant>> = vec![];
        for chunk in chunks {
            compiled_chunks.push(chunk.compile());
        }
        let code = Code {
            chunks: compiled_chunks,
        };

        let mut machine = build_jex_machine(&code);

        let finished_gracefully = machine.start();
        if !finished_gracefully {
            panic!();
        }
        machine.peek_operand().ok().cloned()
    }

    pub fn run_chunk(chunk: TestChunk) -> Option<JexValue> {
        run_chunks(vec![chunk])
    }

    pub fn run_instructions(instructions: Vec<TestInstruction>) -> Option<JexValue> {
        let chunk = TestChunk {
            constants: vec![],
            instructions,
        };
        run_chunk(chunk)
    }
}
