pub const MEMORY_SIZE: usize = 64 * 1024;

pub const MEMORY_LOW_ADDRESS: u16 = 0x0000;
pub const MEMORY_HIGH_ADDRESS: u16 = 0xFFFF;
pub const STACK_LOW_ADDRESS: u16 = 0x100;
pub const STACK_HIGH_ADDRESS: u16 = 0x1FF;
pub const RAM_LOW_ADDRESS: u16 = 0x0000;
pub const RAM_HIGH_ADDRESS: u16 = 0x7FFF;
pub const ROM_LOW_ADDRESS: u16 = 0x8000;
pub const ROM_HIGH_ADDRESS: u16 = 0xFFFF;

pub const NMI: u16 = 0xFFFA;
pub const RESET: u16 = 0xFFFC;
pub const IRQ: u16 = 0xFFFE;

pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Memory {

    pub fn new() -> Memory {
        Memory {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.bytes[address as usize] = value;
    }

    fn read_word(&self, addr: u16) -> u8 {
        let lower = self.read_byte(addr) as u8;
        let upper = self.read_byte(addr + 1) as u8;
        upper << 8 | lower
    }

    fn write_word(&mut self, addr: u16, val: u8) {
        self.write_byte(addr, val);
        self.write_byte(addr, (val >> 8));
    }

}
