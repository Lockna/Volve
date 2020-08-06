use std::ops::Not;
use crate::registers::Registers;
use crate::memory::Memory;
use crate::instruction::{Instruction, AddressingMode, OP_CODES};


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

        while self.is_finished().not() {
            self.step()
        }

    }

    fn is_finished(&mut self) -> bool {
        // cpu shutdowns when invalid opcode is hit
        if (self.fetch_insn() & 0xF == 0x2) {
            true
        } else {
            false
        }
    }

    fn step(&mut self) {
        let bytecode = self.fetch_insn();
        let insn = self.decode_bytecode(bytecode);
        self.execute_insn(insn);
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
