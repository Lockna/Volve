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

}
