use crate::registers::Registers;
use crate::memory::Memory;
use crate::instruction::{Instruction, AddressingMode, OP_CODES};
use std::ops::Index;


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
        self.registers.pc = self.memory.read_word(0xFFFC);
        let mut bytecode;
        let mut insn;

        loop {

            bytecode = self.fetch_insn();
            insn = self.decode_bytecode(bytecode);
            self.execute_insn(insn);

        }

    }

    fn fetch_insn(&mut self) -> u8 {
        self.memory.read_byte(self.registers.pc)
    }

    fn decode_bytecode(&self, bytecode: u8) -> Instruction {
        OP_CODES[bytecode as usize].expect("Invalid instruction hit!")
    }

    pub fn execute_insn(&self, insn: Instruction) {

    }


}
