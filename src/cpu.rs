use crate::registers::Registers;
use crate::memory::Memory;

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



    }

    pub fn fetch_insn(&mut self) -> u8 {
        self.memory.read_byte(self.registers.pc)
    }

    pub fn decode_insn(&self) {
        // unimplemented
    }

    pub fn execute_insn(&self, ) {
        // unimplemented
    }


}
