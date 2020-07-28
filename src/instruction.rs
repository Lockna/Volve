pub enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirectX,
    IndirectIndexedY,
}

pub enum Operand {
    Accumulator,
    XRegister,
    YRegister,
    Address(u16),
    Relative(i8),
    Implied,
}

pub enum OpCode {
    // unimplemented
}

pub struct Instruction {
    pub operand: Operand,
    pub mode: AddressingMode,
    pub opcode: OpCode,
}