use core::str;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;

    #[test]
    fn assembleln_static() {
        let hir = String::from("G0 0.0 0.0");

        let expected_bir = [0u8; 9];

        let bir = Assembler::assembleln(&hir);

        assert_eq!(bir, expected_bir);
    }

    #[test]
    fn assembleln_dynamic() {
        let mut rng = rand::rng();

        for _ in 0..256 {
            // 256 Random Tests

            let opcode = Assembler::INSTRUCTION_CODEC.0[rng.random_range(0..7)];
            let operand_zero: f32 = rng.random_range(0.0..256.0);
            let operand_one: f32 = rng.random_range(0.0..256.0);

            let hir = format!("{} {} {}", opcode, operand_zero, operand_one);

            let expected_bir = {
                let mut expected_bir = [0u8; 9];
                let index = Assembler::INSTRUCTION_CODEC
                    .0
                    .iter()
                    .position(|&x| x == opcode)
                    .expect("Error Parsing Line / Invalid Instruction Type");

                let local_operand_zero = operand_zero.to_le_bytes();
                let local_operand_one = operand_one.to_le_bytes();

                expected_bir[0] = Assembler::INSTRUCTION_CODEC.1[index];
                expected_bir[1..5].copy_from_slice(&local_operand_zero);
                expected_bir[5..9].copy_from_slice(&local_operand_one);
                expected_bir
            };

            let bir = Assembler::assembleln(&hir);

            assert_eq!(bir, expected_bir);
        }
    }

    #[test]
    fn _disassembleln_static() {
        let hir = String::from("G0 0.0 0.0");

        let expected_bytecode = (0, 0.0, 0.0);

        let bytecode = {
            let bir = Assembler::assembleln(&hir);
            let bytecode = Assembler::_disassembleln(&bir); // FIX THE LIFETIME ISSUE
            bytecode
        };

        assert_eq!(bytecode, expected_bytecode);
    }

    #[test]
    fn _disassembleln_dynamic() {
        let mut rng = rand::rng();

        for _ in 0..256 {
            // 256 Random Tests
            let opcode = rng.random_range(0..7);
            let instruction = Assembler::INSTRUCTION_CODEC.0[opcode as usize];
            let operand_zero: f32 = rng.random_range(0.0..256.0);
            let operand_one: f32 = rng.random_range(0.0..256.0);

            let hir = format!("{} {} {}", instruction, operand_zero, operand_one);

            let expected_bytecode = (opcode, operand_zero, operand_one);

            let ir = Assembler::assembleln(&hir);
            let bytecode = Assembler::_disassembleln(&ir);

            assert_eq!(bytecode, expected_bytecode);
        }
    }
}

pub struct Assembler;

impl Assembler {
    const INSTRUCTION_CODEC: ([&str; 7], [u8; 7]) = (
        ["G0", "G1", "G6", "G92", "M3", "M4", "M5"], // Instructions (Modified)
        [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8],         // OpCodes
    );

    pub fn assembleln(hir: &String) -> [u8; 9] {
        // hir / High-level Intermediate Representation

        let mir: Vec<&str> = hir.split_whitespace().collect();
        // mir / Mid-level Intermediate Representation

        let mut bir = [0u8; 9];
        // bir / BInary Representation

        let index = Self::INSTRUCTION_CODEC
            .0
            .iter()
            .position(|&x| x == mir[0])
            .expect("Error Parsing Line / Invalid Instruction Type");

        bir[0] = Self::INSTRUCTION_CODEC.1[index];

        let operand_zero: [u8; 4] = {
            let operand_zero: f32 = mir[1]
                .parse()
                .expect("Error Parsing Line / Invalid Operand Zero");
            operand_zero.to_le_bytes()
        };

        let operand_one: [u8; 4] = {
            let operand_one: f32 = mir[2]
                .parse()
                .expect("Error Parsing Line / Invalid Operand Zero");
            operand_one.to_le_bytes()
        };

        bir[1..5].copy_from_slice(&operand_zero);
        bir[5..9].copy_from_slice(&operand_one);
        bir
    }

    pub fn _disassembleln(bir: &[u8; 9]) -> (u8, f32, f32) {
        // bir / BInary Representation -> Does Not

        let opcode = bir[0];

        let operand_zero = f32::from_le_bytes(bir[1..5].try_into().unwrap());
        let operand_one = f32::from_le_bytes(bir[5..9].try_into().unwrap());

        (opcode, operand_zero, operand_one)
    }
}
