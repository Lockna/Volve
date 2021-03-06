use crate::instruction::{AddressingMode, Instruction, OpCode, OP_CODES};
use crate::memory::Memory;
use crate::registers::{Registers, StatusFlag};
use std::ops::Not;

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
        if (OP_CODES[self.fetch_insn() as usize].is_none()) {
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

    pub fn execute_insn(&mut self, insn: Instruction) {
        match insn.opcode {
            OpCode::ADC => self.adc(insn.mode),
            OpCode::AND => self.and(insn.mode),
            OpCode::ASL => self.asl(insn.mode),
            OpCode::BBR0 => self.bbr(insn.mode, 0),
            OpCode::BBR1 => self.bbr(insn.mode, 1),
            OpCode::BBR2 => self.bbr(insn.mode, 2),
            OpCode::BBR3 => self.bbr(insn.mode, 3),
            OpCode::BBR4 => self.bbr(insn.mode, 4),
            OpCode::BBR5 => self.bbr(insn.mode, 5),
            OpCode::BBR6 => self.bbr(insn.mode, 6),
            OpCode::BBR7 => self.bbr(insn.mode, 7),
            OpCode::BBS0 => self.bbs(insn.mode, 0),
            OpCode::BBS1 => self.bbs(insn.mode, 1),
            OpCode::BBS2 => self.bbs(insn.mode, 2),
            OpCode::BBS3 => self.bbs(insn.mode, 3),
            OpCode::BBS4 => self.bbs(insn.mode, 4),
            OpCode::BBS5 => self.bbs(insn.mode, 5),
            OpCode::BBS6 => self.bbs(insn.mode, 6),
            OpCode::BBS7 => self.bbs(insn.mode, 7),
            OpCode::BCC => self.bcc(insn.mode),
            OpCode::BCS => self.bcs(insn.mode),
            OpCode::BEQ => self.beq(insn.mode),
            OpCode::BIT => self.bit(insn.mode),
            OpCode::BMI => self.bmi(insn.mode),
            OpCode::BNE => self.bne(insn.mode),
            OpCode::BPL => self.bpl(insn.mode),
            OpCode::BRK => self.brk(insn.mode),
            OpCode::BVC => self.bvc(insn.mode),
            OpCode::BVS => self.bvs(insn.mode),
            OpCode::CLC => self.clc(insn.mode),
            OpCode::CLD => self.cld(insn.mode),
            OpCode::CLI => self.cli(insn.mode),
            OpCode::CLV => self.clv(insn.mode),
            OpCode::CMP => self.cmp(insn.mode),
            OpCode::CPX => self.cpx(insn.mode),
            OpCode::CPY => self.cpy(insn.mode),
            OpCode::DCP => self.dcp(insn.mode),
            OpCode::DEC => self.dec(insn.mode),
            OpCode::DEX => self.dex(insn.mode),
            OpCode::DEY => self.dey(insn.mode),
            OpCode::EOR => self.eor(insn.mode),
            OpCode::INC => self.inc(insn.mode),
            OpCode::INX => self.inx(insn.mode),
            OpCode::INY => self.iny(insn.mode),
            OpCode::ISB => self.isb(insn.mode),
            OpCode::JMP => self.jmp(insn.mode),
            OpCode::JSR => self.jsr(insn.mode),
            OpCode::LAX => self.lax(insn.mode),
            OpCode::LDA => self.lda(insn.mode),
            OpCode::LDX => self.ldx(insn.mode),
            OpCode::LDY => self.ldy(insn.mode),
            OpCode::LSR => self.lsr(insn.mode),
            OpCode::NOP => self.nop(insn.mode),
            OpCode::ORA => self.ora(insn.mode),
            OpCode::PHA => self.pha(insn.mode),
            OpCode::PHP => self.php(insn.mode),
            OpCode::PLA => self.pla(insn.mode),
            OpCode::PLP => self.plp(insn.mode),
            OpCode::RLA => self.rla(insn.mode),
            OpCode::RMB0 => self.rmb0(insn.mode),
            OpCode::RMB1 => self.rmb1(insn.mode),
            OpCode::RMB2 => self.rmb2(insn.mode),
            OpCode::RMB3 => self.rmb3(insn.mode),
            OpCode::RMB4 => self.rmb4(insn.mode),
            OpCode::RMB5 => self.rmb5(insn.mode),
            OpCode::RMB6 => self.rmb6(insn.mode),
            OpCode::RMB7 => self.rmb7(insn.mode),
            OpCode::ROL => self.rol(insn.mode),
            OpCode::ROR => self.ror(insn.mode),
            OpCode::RRA => self.rra(insn.mode),
            OpCode::RTI => self.rti(insn.mode),
            OpCode::RTS => self.rts(insn.mode),
            OpCode::SAX => self.sax(insn.mode),
            OpCode::SBC => self.sbc(insn.mode),
            OpCode::SEC => self.sec(insn.mode),
            OpCode::SED => self.sed(insn.mode),
            OpCode::SEI => self.sei(insn.mode),
            OpCode::SLO => self.slo(insn.mode),
            OpCode::SRE => self.sre(insn.mode),
            OpCode::STA => self.sta(insn.mode),
            OpCode::STX => self.stx(insn.mode),
            OpCode::STY => self.sty(insn.mode),
            OpCode::TAX => self.tax(insn.mode),
            OpCode::TAY => self.tay(insn.mode),
            OpCode::TRB => self.trb(insn.mode),
            OpCode::TSB => self.tsb(insn.mode),
            OpCode::TSX => self.tsx(insn.mode),
            OpCode::TXA => self.txa(insn.mode),
            OpCode::TXS => self.txs(insn.mode),
            OpCode::TYA => self.tya(insn.mode),
        }
    }
    // TODO: implement the instructions
    fn adc(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute => {
                let operand = self
                    .memory
                    .read_byte(self.memory.read_word(self.registers.pc + 1));

                self.registers.a += operand + (self.registers.get_flag(StatusFlag::Carry) as u8);
            }

            AddressingMode::AbsoluteX => {
                let specified_addr = self
                    .memory
                    .read_byte(self.memory.read_word(self.registers.pc + 1) as u16);
                let sum_addr = specified_addr + self.registers.x;
                let operand = self.memory.read_byte(sum_addr as u16);

                self.registers.a += operand + (self.registers.get_flag(StatusFlag::Carry) as u8);
            }

            AddressingMode::AbsoluteY => {
                let specified_addr = self
                    .memory
                    .read_byte(self.memory.read_word(self.registers.pc + 1) as u16);
                let sum_addr = specified_addr + self.registers.y;
                let operand = self.memory.read_byte(sum_addr as u16);

                self.registers.a += operand + (self.registers.get_flag(StatusFlag::Carry) as u8);
            }

            AddressingMode::Immediate => {
                let operand = self.memory.read_byte(self.registers.pc + 1);

                // i dont know if it is right but i dont care
                self.registers.a += operand + self.registers.get_flag(StatusFlag::Carry) as u8;
            }

            AddressingMode::Indirect => {
                let address = self.memory.read_byte(self.registers.pc + 1) as u16;

                let operand = self.memory.read_byte(address);

                self.registers.a += operand + self.registers.get_flag(StatusFlag::Carry) as u8;
            }

            AddressingMode::IndexedIndirectX => {
                let specified_addr = self.memory.read_word(self.registers.pc + 1);

                let operand = self
                    .memory
                    .read_byte(specified_addr + self.registers.x as u16);

                self.registers.a += operand + self.registers.get_flag(StatusFlag::Carry) as u8;
            }

            AddressingMode::IndirectIndexedY => {
                let specified_addr = self.memory.read_word(self.registers.pc + 1);

                let operand = self
                    .memory
                    .read_byte(specified_addr + self.registers.y as u16);

                self.registers.a += operand + self.registers.get_flag(StatusFlag::Carry) as u8;
            }

            AddressingMode::ZeroPage => {
                let operand = self
                    .memory
                    .read_byte(self.memory.read_byte(self.registers.pc + 1) as u16);

                self.registers.a += operand + (self.registers.get_flag(StatusFlag::Carry) as u8);
            }

            AddressingMode::ZeroPageX => {
                let specified_addr = self.memory.read_byte(self.registers.pc + 1) as u16;
                let sum_addr = specified_addr + self.registers.x as u16;

                self.registers.a += self.memory.read_byte(sum_addr)
                    + (self.registers.get_flag(StatusFlag::Carry) as u8);
            }

            AddressingMode::ZeroPageY => {
                let specified_addr = self.memory.read_byte(self.registers.pc + 1) as u16;
                let sum_addr = specified_addr + self.registers.y as u16;

                self.registers.a += self.memory.read_byte(sum_addr)
                    + (self.registers.get_flag(StatusFlag::Carry) as u8);
            }

            _ => unreachable!("Invalid addressing mode {:?} for adc", mode),
        }
    }

    fn and(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute => {
                let operand = self
                    .memory
                    .read_byte(self.memory.read_word(self.registers.pc + 1));

                self.registers.a &= operand;
            }

            AddressingMode::AbsoluteX => {
                let specified_addr = self
                    .memory
                    .read_byte(self.memory.read_word(self.registers.pc + 1) as u16);
                let sum_addr = specified_addr + self.registers.x;
                let operand = self.memory.read_byte(sum_addr as u16);

                self.registers.a &= operand;
            }

            AddressingMode::AbsoluteY => {
                let specified_addr = self
                    .memory
                    .read_byte(self.memory.read_word(self.registers.pc + 1) as u16);
                let sum_addr = specified_addr + self.registers.y;
                let operand = self.memory.read_byte(sum_addr as u16);

                self.registers.a += operand;
            }

            AddressingMode::Immediate => {
                let operand = self.memory.read_byte(self.registers.pc + 1);

                // i dont know if it is right but i dont care
                self.registers.a &= operand;
            }

            AddressingMode::Indirect => {
                let address = self.memory.read_byte(self.registers.pc + 1) as u16;

                let operand = self.memory.read_byte(address);

                self.registers.a &= operand;
            }

            AddressingMode::IndexedIndirectX => {
                let specified_addr = self.memory.read_word(self.registers.pc + 1);

                let operand = self
                    .memory
                    .read_byte(specified_addr + self.registers.x as u16);

                self.registers.a &= operand;
            }

            AddressingMode::IndirectIndexedY => {
                let specified_addr = self.memory.read_word(self.registers.pc + 1);

                let operand = self
                    .memory
                    .read_byte(specified_addr + self.registers.y as u16);

                self.registers.a &= operand;
            }

            AddressingMode::ZeroPage => {
                let operand = self
                    .memory
                    .read_byte(self.memory.read_byte(self.registers.pc + 1) as u16);

                self.registers.a &= operand;
            }

            AddressingMode::ZeroPageX => {
                let specified_addr = self.memory.read_byte(self.registers.pc + 1) as u16;
                let sum_addr = specified_addr + self.registers.x as u16;

                self.registers.a &= self.memory.read_byte(sum_addr);
            }

            AddressingMode::ZeroPageY => {
                let specified_addr = self.memory.read_byte(self.registers.pc + 1) as u16;
                let sum_addr = specified_addr + self.registers.y as u16;

                self.registers.a &= self.memory.read_byte(sum_addr);
            }

            _ => unreachable!("Invalid addressing mode {:?} for and", mode),
        }
    }

    fn asl(&mut self, mode: AddressingMode) {
        match mode {
            // it should be Absolute RMW but i dont know the difference
            AddressingMode::Absolute => {
                let addr = self.memory.read_word(self.registers.pc + 1);

                self.memory
                    .write_byte(addr, self.memory.read_byte(addr) << 1);
            }

            AddressingMode::AbsoluteX => {
                let specified_addr = self.memory.read_word(self.registers.pc + 1) as u16;
                let sum_addr = specified_addr + self.registers.x as u16;

                self.memory
                    .write_byte(sum_addr, self.memory.read_byte(sum_addr) << 1);
            }

            AddressingMode::Accumulator => {
                self.registers.a <<= 1;
            }

            AddressingMode::ZeroPage => {
                let addr = self.memory.read_byte(self.registers.pc + 1) as u16;

                self.memory
                    .write_byte(addr, self.memory.read_byte(addr) << 1);
            }

            AddressingMode::ZeroPageX => {
                let addr = (self.memory.read_byte(self.registers.pc + 1) + self.registers.x) as u16;

                self.memory
                    .write_byte(addr, self.memory.read_byte(addr) << 1);
            }

            _ => unreachable!("Invalid addressing mode {:?} for asl", mode),
        }
    }

    fn bbr(&mut self, mode: AddressingMode, bit: u8) {
        if (self.registers.p & 1 << bit) == 0 {
            let rel_jmp_addr = self.memory.read_byte(self.registers.pc + 1) as i8;
            if rel_jmp_addr < 0 {
                self.registers.pc -= rel_jmp_addr as u16;
            } else {
                self.registers.pc += rel_jmp_addr as u16;
            }
        }
    }

    fn bbs(&mut self, mode: AddressingMode, bit: u8) {
        if (self.registers.p & 1 << bit) != 0 {
            let rel_jmp_addr = self.memory.read_byte(self.registers.pc + 1) as i8;
            if rel_jmp_addr < 0 {
                self.registers.pc -= rel_jmp_addr as u16;
            } else {
                self.registers.pc += rel_jmp_addr as u16;
            }
        }
    }

    fn bcc(&mut self, mode: AddressingMode) {
        if !self.registers.get_flag(StatusFlag::Carry) {
            let rel_jmp_addr = self.memory.read_byte(self.registers.pc + 1) as i8;
            if rel_jmp_addr < 0 {
                self.registers.pc -= rel_jmp_addr as u16;
            } else {
                self.registers.pc += rel_jmp_addr as u16;
            }
        }
    }

    fn bcs(&mut self, mode: AddressingMode) {
        if self.registers.get_flag(StatusFlag::Carry) {
            let rel_jmp_addr = self.memory.read_byte(self.registers.pc + 1) as i8;
            if rel_jmp_addr < 0 {
                self.registers.pc -= rel_jmp_addr as u16;
            } else {
                self.registers.pc += rel_jmp_addr as u16;
            }
        }
    }

    fn beq(&mut self, mode: AddressingMode) {}

    fn bit(&mut self, mode: AddressingMode) {}

    fn bmi(&mut self, mode: AddressingMode) {}

    fn bne(&mut self, mode: AddressingMode) {}

    fn bpl(&mut self, mode: AddressingMode) {}

    fn brk(&mut self, mode: AddressingMode) {}

    fn bvc(&mut self, mode: AddressingMode) {}

    fn bvs(&mut self, mode: AddressingMode) {}

    fn clc(&mut self, mode: AddressingMode) {}

    fn cld(&mut self, mode: AddressingMode) {}

    fn cli(&mut self, mode: AddressingMode) {}

    fn clv(&mut self, mode: AddressingMode) {}

    fn cmp(&mut self, mode: AddressingMode) {}

    fn cpx(&mut self, mode: AddressingMode) {}

    fn cpy(&mut self, mode: AddressingMode) {}

    fn dcp(&mut self, mode: AddressingMode) {}

    fn dec(&mut self, mode: AddressingMode) {}

    fn dex(&mut self, mode: AddressingMode) {}
    fn dey(&mut self, mode: AddressingMode) {}
    fn eor(&mut self, mode: AddressingMode) {}
    fn inc(&mut self, mode: AddressingMode) {}
    fn inx(&mut self, mode: AddressingMode) {}
    fn iny(&mut self, mode: AddressingMode) {}
    fn isb(&mut self, mode: AddressingMode) {}
    fn jmp(&mut self, mode: AddressingMode) {}
    fn jsr(&mut self, mode: AddressingMode) {}
    fn lax(&mut self, mode: AddressingMode) {}
    fn lda(&mut self, mode: AddressingMode) {}
    fn ldx(&mut self, mode: AddressingMode) {}
    fn ldy(&mut self, mode: AddressingMode) {}
    fn lsr(&mut self, mode: AddressingMode) {}
    fn nop(&mut self, mode: AddressingMode) {}
    fn ora(&mut self, mode: AddressingMode) {}
    fn pha(&mut self, mode: AddressingMode) {}
    fn php(&mut self, mode: AddressingMode) {}
    fn pla(&mut self, mode: AddressingMode) {}
    fn plp(&mut self, mode: AddressingMode) {}
    fn rla(&mut self, mode: AddressingMode) {}
    fn rmb0(&mut self, mode: AddressingMode) {}
    fn rmb1(&mut self, mode: AddressingMode) {}
    fn rmb2(&mut self, mode: AddressingMode) {}
    fn rmb3(&mut self, mode: AddressingMode) {}
    fn rmb4(&mut self, mode: AddressingMode) {}
    fn rmb5(&mut self, mode: AddressingMode) {}
    fn rmb6(&mut self, mode: AddressingMode) {}
    fn rmb7(&mut self, mode: AddressingMode) {}
    fn rol(&mut self, mode: AddressingMode) {}
    fn ror(&mut self, mode: AddressingMode) {}
    fn rra(&mut self, mode: AddressingMode) {}
    fn rti(&mut self, mode: AddressingMode) {}
    fn rts(&mut self, mode: AddressingMode) {}
    fn sax(&mut self, mode: AddressingMode) {}
    fn sbc(&mut self, mode: AddressingMode) {}
    fn sec(&mut self, mode: AddressingMode) {}
    fn sed(&mut self, mode: AddressingMode) {}
    fn sei(&mut self, mode: AddressingMode) {}
    fn slo(&mut self, mode: AddressingMode) {}
    fn sre(&mut self, mode: AddressingMode) {}
    fn sta(&mut self, mode: AddressingMode) {}
    fn stx(&mut self, mode: AddressingMode) {}
    fn sty(&mut self, mode: AddressingMode) {}
    fn tax(&mut self, mode: AddressingMode) {}
    fn tay(&mut self, mode: AddressingMode) {}
    fn trb(&mut self, mode: AddressingMode) {}
    fn tsb(&mut self, mode: AddressingMode) {}
    fn tsx(&mut self, mode: AddressingMode) {}
    fn txa(&mut self, mode: AddressingMode) {}
    fn txs(&mut self, mode: AddressingMode) {}
    fn tya(&mut self, mode: AddressingMode) {}
}
