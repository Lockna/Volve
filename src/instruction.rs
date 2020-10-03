#[derive(Copy, Debug, Clone)]
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
    AbsoluteIndexedIndirect,
    AbsoluteIndirect,
    Indirect,
    IndexedIndirectX,
    IndirectIndexedY,
}

#[derive(Copy, Debug, Clone)]
pub enum OpCode {
    ADC,
    AND,
    ASL,
    BBR0,
    BBR1,
    BBR2,
    BBR3,
    BBR4,
    BBR5,
    BBR6,
    BBR7,
    BBS0,
    BBS1,
    BBS2,
    BBS3,
    BBS4,
    BBS5,
    BBS6,
    BBS7,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    ISB,
    JMP,
    JSR,
    LAX,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    RMB0,
    RMB1,
    RMB2,
    RMB3,
    RMB4,
    RMB5,
    RMB6,
    RMB7,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SAX,
    SBC,
    SEC,
    SED,
    SEI,
    SLO,
    SRE,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TRB,
    TSB,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Copy, Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub mode: AddressingMode,
}

pub static OP_CODES: [Option<Instruction>; 256] = [
    Some(Instruction {
        opcode: OpCode::BRK,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::TSB,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::ASL,
        mode: AddressingMode::ZeroPage,
    }),
    // 7
    Some(Instruction {
        opcode: OpCode::RMB0,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::PHP,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::ASL,
        mode: AddressingMode::Accumulator,
    }),
    // 0b
    None,
    Some(Instruction {
        opcode: OpCode::TSB,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::ASL,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::BBR0,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::BPL,
        mode: AddressingMode::Relative,
    }),
    // 11
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::IndirectIndexedY,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::Indirect,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::TRB,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::ASL,
        mode: AddressingMode::ZeroPageX,
    }),
    // 17
    Some(Instruction {
        opcode: OpCode::RMB1,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::CLC,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::AbsoluteY,
    }),
    // 1a
    Some(Instruction {
        opcode: OpCode::INC,
        mode: AddressingMode::Accumulator,
    }),
    // 1b
    None,
    // 1c
    Some(Instruction {
        opcode: OpCode::TRB,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::ORA,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::ASL,
        mode: AddressingMode::AbsoluteX,
    }),
    // 1f
    Some(Instruction {
        opcode: OpCode::BBR1,
        mode: AddressingMode::Relative,
    }),
    // 20
    Some(Instruction {
        opcode: OpCode::JSR,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::BIT,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::ROL,
        mode: AddressingMode::ZeroPage,
    }),
    // 27
    Some(Instruction {
        opcode: OpCode::RMB2,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::PLP,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::Immediate,
    }),
    // 2a
    Some(Instruction {
        opcode: OpCode::ROL,
        mode: AddressingMode::Accumulator,
    }),
    // 2b
    None,
    Some(Instruction {
        opcode: OpCode::BIT,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::ROL,
        mode: AddressingMode::Absolute,
    }),
    // 2f
    Some(Instruction {
        opcode: OpCode::BBR2,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::BMI,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::IndirectIndexedY,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::Indirect,
    }),
    // 33
    None,
    Some(Instruction {
        opcode: OpCode::BIT,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::ROL,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::RMB3,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::SEC,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::AbsoluteY,
    }),
    // 3a
    Some(Instruction {
        opcode: OpCode::DEC,
        mode: AddressingMode::Accumulator,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::AND,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::ROL,
        mode: AddressingMode::AbsoluteX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::RTI,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::LSR,
        mode: AddressingMode::ZeroPage,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::PHA,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::LSR,
        mode: AddressingMode::Accumulator,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::JMP,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::LSR,
        mode: AddressingMode::Absolute,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::BVC,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::IndirectIndexedY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::LSR,
        mode: AddressingMode::ZeroPageX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CLI,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::AbsoluteX,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::EOR,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::LSR,
        mode: AddressingMode::AbsoluteX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::RTS,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::ROR,
        mode: AddressingMode::ZeroPage,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::PLA,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::ROR,
        mode: AddressingMode::Accumulator,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::JMP,
        mode: AddressingMode::Indirect,
    }),
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::ROR,
        mode: AddressingMode::Absolute,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::BVS,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::IndirectIndexedY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::ROR,
        mode: AddressingMode::ZeroPageX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::SEI,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::AbsoluteY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::ADC,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::ROR,
        mode: AddressingMode::Implied,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::STA,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::STY,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::STA,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::STX,
        mode: AddressingMode::ZeroPage,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::DEY,
        mode: AddressingMode::Implied,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::TXA,
        mode: AddressingMode::Implied,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::STY,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::STY,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::STX,
        mode: AddressingMode::Absolute,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::BCC,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::STA,
        mode: AddressingMode::IndirectIndexedY,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::STY,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::STA,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::STX,
        mode: AddressingMode::ZeroPageY,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::TYA,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::STA,
        mode: AddressingMode::AbsoluteY,
    }),
    Some(Instruction {
        opcode: OpCode::TXS,
        mode: AddressingMode::Implied,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::STA,
        mode: AddressingMode::AbsoluteX,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::LDY,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::IndirectIndexedY,
    }),
    Some(Instruction {
        opcode: OpCode::LDX,
        mode: AddressingMode::Immediate,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::LDY,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::LDX,
        mode: AddressingMode::ZeroPage,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::TAY,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::TAX,
        mode: AddressingMode::Implied,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::LDY,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::LDX,
        mode: AddressingMode::Absolute,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::BCS,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::IndirectIndexedY,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::LDY,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::LDX,
        mode: AddressingMode::ZeroPageY,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CLV,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::AbsoluteY,
    }),
    Some(Instruction {
        opcode: OpCode::TSX,
        mode: AddressingMode::Implied,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::LDY,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::LDA,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::LDX,
        mode: AddressingMode::AbsoluteY,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CPY,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::CPY,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::DEC,
        mode: AddressingMode::ZeroPage,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::INY,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::DEX,
        mode: AddressingMode::Implied,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CPY,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::DEC,
        mode: AddressingMode::Absolute,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::BNE,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::IndirectIndexedY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::DEC,
        mode: AddressingMode::ZeroPageX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CLD,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::AbsoluteY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::CMP,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::DEC,
        mode: AddressingMode::AbsoluteX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CPX,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::IndexedIndirectX,
    }),
    None,
    None,
    Some(Instruction {
        opcode: OpCode::CPX,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::ZeroPage,
    }),
    Some(Instruction {
        opcode: OpCode::INC,
        mode: AddressingMode::ZeroPage,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::INX,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::Immediate,
    }),
    Some(Instruction {
        opcode: OpCode::NOP,
        mode: AddressingMode::Implied,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::CPX,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::Absolute,
    }),
    Some(Instruction {
        opcode: OpCode::INC,
        mode: AddressingMode::Absolute,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::BEQ,
        mode: AddressingMode::Relative,
    }),
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::IndirectIndexedY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::ZeroPageX,
    }),
    Some(Instruction {
        opcode: OpCode::INC,
        mode: AddressingMode::ZeroPageX,
    }),
    None,
    Some(Instruction {
        opcode: OpCode::SED,
        mode: AddressingMode::Implied,
    }),
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::AbsoluteY,
    }),
    None,
    None,
    None,
    Some(Instruction {
        opcode: OpCode::SBC,
        mode: AddressingMode::AbsoluteX,
    }),
    Some(Instruction {
        opcode: OpCode::INC,
        mode: AddressingMode::AbsoluteX,
    }),
    None,
];
