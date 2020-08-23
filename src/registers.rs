pub enum StatusFlag {
    Carry = 1 << 0,
    Zero = 1 << 1,
    NoInterrupts = 1 << 2,
    Decimal = 1 << 3,
    Break = 1 << 4,
    Unused = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub p: u8,
    pub sp: u8,
    pub pc: u16,
}

impl Registers {

    
    pub fn set_flag(&mut self, flag: StatusFlag, mode: bool) {
        if mode {
            self.p |= flag as u8;
        } else {
            self.p &= !(flag as u8);
        }
    }

    pub fn get_flag(&mut self, flag: StatusFlag) -> bool {
        (self.p & flag as u8) != 0
    }


    pub fn new() -> Registers {

        Registers {

            a: 0,
            x: 0,
            y: 0,

            p: 0x34,
            sp: 0xFF,
            pc: 0,

        }

    }

}
