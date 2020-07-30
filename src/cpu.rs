use crate::registers::Registers;
use crate::memory::Memory;
use crate::instruction::{Instruction, OpCode, Operand, AddressingMode};


pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
}

impl Cpu {

    pub fn new() -> Cpu {

        Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
        }

    }

    pub fn run(&mut self) {
        self.registers.pc = self.memory[0xFFFD] * 256 + self.memory[0xFFFC];
        let mut bytecode;
        let mut insn;

        loop {

            bytecode = self.fetch_insn();
            insn = self.decode_opcode(opcode_byte);
            self.execute_insn(insn);

        }

    }

    fn fetch_insn(&mut self) -> u8 {
        self.memory.read_byte(self.registers.pc)
    }

    fn decode_bytecode(&self, bytecode: u8) -> Instruction {

        match bytecode {

            0x00 => Instruction { operand: Operand::None, mode: AddressingMode::Stack, opcode: OpCode::BRK },
            0x01 => Instruction { operand: Operand::XRegister, mode: AddressingMode::IndexedIndirectX, opcode: OpCode::ORA },
            // do the same for the rest
            _ => None
        }

    }

    pub fn execute_insn(&self, insn: Instruction) {

    }


}
