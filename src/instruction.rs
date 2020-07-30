pub enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Stack,
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
    None,
}

pub enum OpCode {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DCP, DEC, DEX, DEY, EOR, INC, INX, INY, ISB, JMP, JSR, LAX,
    LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, RLA, ROL, ROR, RRA, RTI, RTS,
    SAX, SBC, SEC, SED, SEI, SLO, SRE, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,
    ___
}

pub struct Instruction {
    pub operand: Operand,
    pub mode: AddressingMode,
    pub opcode: OpCode,
}