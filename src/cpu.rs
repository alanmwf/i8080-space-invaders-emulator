use std::mem::swap;

use crate::{
    memory::Memory,
    opcode::OpCode,
    register::{Flag, Register}, machine::Machine,
};

pub struct CPU {
    register: Register,
    memory: Memory,
    interrupt_enabled: bool,
    halted: bool,
    pub show_debug_log: bool,
}

impl CPU {
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn emulate(&mut self, machine: &mut impl Machine) -> u8 {
        if self.halted { return 0; }
        let data = self.read_immediate();
        let opcode = OpCode(data);
        let opcode = match opcode.0 {
            0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => OpCode(0x00),
            0xCB => OpCode(0xC3),
            0xD9 => OpCode(0xC9),
            0xDD | 0xED | 0xFD => OpCode(0xCD),
            _ => opcode,
        };
        let mut cycle = opcode.cycles();

        if self.show_debug_log {
            println!(
                "{} PC={:04x} SP={:04x} A={:02x} F={:02x} B={:02x} C={:02x} D={:02x} E={:02x} H={:02x} L={:02x}",
                opcode.menmonic(),
                self.register.program_counter.wrapping_sub(1),
                self.register.stack_pointer,
                self.register.a,
                self.register.flags,
                self.register.b,
                self.register.c,
                self.register.d,
                self.register.e,
                self.register.h,
                self.register.l,
            );
        }

        match opcode.0 {
            0x00 => {}

            // MOV Instruction
            0x40 => {}
            0x41 => self.register.b = self.register.c,
            0x42 => self.register.b = self.register.d,
            0x43 => self.register.b = self.register.e,
            0x44 => self.register.b = self.register.h,
            0x45 => self.register.b = self.register.l,
            0x46 => self.register.b = self.get_hl_data(),
            0x47 => self.register.b = self.register.a,
            0x48 => self.register.c = self.register.b,
            0x49 => {}
            0x4A => self.register.c = self.register.d,
            0x4B => self.register.c = self.register.e,
            0x4C => self.register.c = self.register.h,
            0x4D => self.register.c = self.register.l,
            0x4E => self.register.c = self.get_hl_data(),
            0x4F => self.register.c = self.register.a,
            0x50 => self.register.d = self.register.b,
            0x51 => self.register.d = self.register.c,
            0x52 => {}
            0x53 => self.register.d = self.register.e,
            0x54 => self.register.d = self.register.h,
            0x55 => self.register.d = self.register.l,
            0x56 => self.register.d = self.get_hl_data(),
            0x57 => self.register.d = self.register.a,
            0x58 => self.register.e = self.register.b,
            0x59 => self.register.e = self.register.c,
            0x5A => self.register.e = self.register.d,
            0x5B => {}
            0x5C => self.register.e = self.register.h,
            0x5D => self.register.e = self.register.l,
            0x5E => self.register.e = self.get_hl_data(),
            0x5F => self.register.e = self.register.a,
            0x60 => self.register.h = self.register.b,
            0x61 => self.register.h = self.register.c,
            0x62 => self.register.h = self.register.d,
            0x63 => self.register.h = self.register.e,
            0x64 => {}
            0x65 => self.register.h = self.register.l,
            0x66 => self.register.h = self.get_hl_data(),
            0x67 => self.register.h = self.register.a,
            0x68 => self.register.l = self.register.b,
            0x69 => self.register.l = self.register.c,
            0x6A => self.register.l = self.register.d,
            0x6B => self.register.l = self.register.e,
            0x6C => self.register.l = self.register.h,
            0x6D => {}
            0x6E => self.register.l = self.get_hl_data(),
            0x6F => self.register.l = self.register.a,
            0x70 => self.set_hl_data(self.register.b),
            0x71 => self.set_hl_data(self.register.c),
            0x72 => self.set_hl_data(self.register.d),
            0x73 => self.set_hl_data(self.register.e),
            0x74 => self.set_hl_data(self.register.h),
            0x75 => self.set_hl_data(self.register.l),
            0x77 => self.set_hl_data(self.register.a),
            0x78 => self.register.a = self.register.b,
            0x79 => self.register.a = self.register.c,
            0x7A => self.register.a = self.register.d,
            0x7B => self.register.a = self.register.e,
            0x7C => self.register.a = self.register.h,
            0x7D => self.register.a = self.register.l,
            0x7E => self.register.a = self.get_hl_data(),
            0x7F => {}

            // MVI Move Immediate Data
            0x06 => self.register.b = self.read_immediate(),
            0x0E => self.register.c = self.read_immediate(),
            0x16 => self.register.d = self.read_immediate(),
            0x1E => self.register.e = self.read_immediate(),
            0x26 => self.register.h = self.read_immediate(),
            0x2E => self.register.l = self.read_immediate(),
            0x36 => {
                let data = self.read_immediate();
                self.set_hl_data(data);
            }
            0x3E => self.register.a = self.read_immediate(),

            // LXI Load Register Pair Immediate
            0x01 => {
                let data = self.read_word_immediate();
                self.register.set_bc(data);
            }
            0x11 => {
                let data = self.read_word_immediate();
                self.register.set_de(data);
            }
            0x21 => {
                let data = self.read_word_immediate();
                self.register.set_hl(data);
            }
            0x31 => {
                let data = self.read_word_immediate();
                self.register.stack_pointer = data;
            }

            // LDA Load Accumulator Direct
            0x3A => {
                let address = self.read_word_immediate();
                let data = self.memory.read(address);
                self.register.a = data;
            }

            // STA Store Accumulator Direct
            0x32 => {
                let address = self.read_word_immediate();
                self.memory.write(address, self.register.a);
            }

            // LHLD Load Hand L Direct
            0x2A => {
                let address = self.read_word_immediate();
                let data = self.memory.read_word(address);
                self.register.set_hl(data);
            }

            // SHLD Store H and L Direct
            0x22 => {
                let address = self.read_word_immediate();
                self.memory.write_word(address, self.register.get_hl());
            }

            // LDAX Load Accumulator
            0x0A => self.register.a = self.memory.read(self.register.get_bc()),
            0x1A => self.register.a = self.memory.read(self.register.get_de()),

            // STAX Store Accumulator
            0x02 => self.memory.write(self.register.get_bc(), self.register.a),
            0x12 => self.memory.write(self.register.get_de(), self.register.a),

            // XCHG Exchange Registers
            0xEB => {
                swap(&mut self.register.h, &mut self.register.d);
                swap(&mut self.register.l, &mut self.register.e);
            }

            // ADD Add Register or Memory to Accumulator
            0x80 => self.add(self.register.b),
            0x81 => self.add(self.register.c),
            0x82 => self.add(self.register.d),
            0x83 => self.add(self.register.e),
            0x84 => self.add(self.register.h),
            0x85 => self.add(self.register.l),
            0x86 => self.add(self.get_hl_data()),
            0x87 => self.add(self.register.a),

            // ADI Add Immediate To Accumulator
            0xC6 => {
                let data = self.read_immediate();
                self.add(data)
            }

            // ADC ADD Register or Memory To Accumulator With Carry
            0x88 => self.adc(self.register.b),
            0x89 => self.adc(self.register.c),
            0x8A => self.adc(self.register.d),
            0x8B => self.adc(self.register.e),
            0x8C => self.adc(self.register.h),
            0x8D => self.adc(self.register.l),
            0x8E => self.adc(self.get_hl_data()),
            0x8F => self.adc(self.register.a),

            // ACI Add Immediate to Accumulator With Carry
            0xCE => {
                let data = self.read_immediate();
                self.adc(data);
            }

            // SUB Subtract Register or Memory
            0x90 => self.sub(self.register.b),
            0x91 => self.sub(self.register.c),
            0x92 => self.sub(self.register.d),
            0x93 => self.sub(self.register.e),
            0x94 => self.sub(self.register.h),
            0x95 => self.sub(self.register.l),
            0x96 => self.sub(self.get_hl_data()),
            0x97 => self.sub(self.register.a),

            // SUI Subtract Immediate From Accumulator
            0xD6 => {
                let data = self.read_immediate();
                self.sub(data);
            }

            // SBB Subtract Register or Memory From Accumulator With Borrow
            0x98 => self.sbb(self.register.b),
            0x99 => self.sbb(self.register.c),
            0x9A => self.sbb(self.register.d),
            0x9B => self.sbb(self.register.e),
            0x9C => self.sbb(self.register.h),
            0x9D => self.sbb(self.register.l),
            0x9E => self.sbb(self.get_hl_data()),
            0x9F => self.sbb(self.register.a),

            // SBI Subtract Immediate from Accumulator With Borrow
            0xDE => {
                let data = self.read_immediate();
                self.sbb(data);
            }

            // INR Increment Register or Memory
            0x04 => self.register.b = self.inr(self.register.b),
            0x0C => self.register.c = self.inr(self.register.c),
            0x14 => self.register.d = self.inr(self.register.d),
            0x1C => self.register.e = self.inr(self.register.e),
            0x24 => self.register.h = self.inr(self.register.h),
            0x2C => self.register.l = self.inr(self.register.l),
            0x34 => {
                let data = self.get_hl_data();
                let new_data = self.inr(data);
                self.set_hl_data(new_data);
            }
            0x3C => self.register.a = self.inr(self.register.a),

            // DCR Decrement Register or Memory
            0x05 => self.register.b = self.dcr(self.register.b),
            0x0D => self.register.c = self.dcr(self.register.c),
            0x15 => self.register.d = self.dcr(self.register.d),
            0x1D => self.register.e = self.dcr(self.register.e),
            0x25 => self.register.h = self.dcr(self.register.h),
            0x2D => self.register.l = self.dcr(self.register.l),
            0x35 => {
                let data = self.get_hl_data();
                let new_data = self.dcr(data);
                self.set_hl_data(new_data);
            }
            0x3D => self.register.a = self.dcr(self.register.a),

            // INX Increment Register Pair
            0x03 => {
                let new_bc = self.register.get_bc().wrapping_add(1);
                self.register.set_bc(new_bc);
            }
            0x13 => {
                let new_de = self.register.get_de().wrapping_add(1);
                self.register.set_de(new_de);
            }
            0x23 => {
                let new_hl = self.register.get_hl().wrapping_add(1);
                self.register.set_hl(new_hl);
            }
            0x33 => self.register.stack_pointer = self.register.stack_pointer.wrapping_add(1),

            // DCX Decrement Register Pair
            0x0B => {
                let new_bc = self.register.get_bc().wrapping_sub(1);
                self.register.set_bc(new_bc);
            }
            0x1B => {
                let new_de = self.register.get_de().wrapping_sub(1);
                self.register.set_de(new_de);
            }
            0x2B => {
                let new_hl = self.register.get_hl().wrapping_sub(1);
                self.register.set_hl(new_hl);
            }
            0x3B => self.register.stack_pointer = self.register.stack_pointer.wrapping_sub(1),

            // DAD Double Add
            0x09 => self.dad(self.register.get_bc()),
            0x19 => self.dad(self.register.get_de()),
            0x29 => self.dad(self.register.get_hl()),
            0x39 => self.dad(self.register.stack_pointer),

            // DAA Decimal Adjust Accumulator
            0x27 => self.daa(),

            // ANA Logical and Register or Memory With Accumulator
            0xA0 => self.ana(self.register.b),
            0xA1 => self.ana(self.register.c),
            0xA2 => self.ana(self.register.d),
            0xA3 => self.ana(self.register.e),
            0xA4 => self.ana(self.register.h),
            0xA5 => self.ana(self.register.l),
            0xA6 => self.ana(self.get_hl_data()),
            0xA7 => self.ana(self.register.a),

            // ANI And Immediate With Accumulator
            0xE6 => {
                let data = self.read_immediate();
                self.ana(data);
            }

            // ORA Logical or Register or Memory With Accumulator
            0xB0 => self.ora(self.register.b),
            0xB1 => self.ora(self.register.c),
            0xB2 => self.ora(self.register.d),
            0xB3 => self.ora(self.register.e),
            0xB4 => self.ora(self.register.h),
            0xB5 => self.ora(self.register.l),
            0xB6 => self.ora(self.get_hl_data()),
            0xB7 => self.ora(self.register.a),

            // ORI OR Immediate With Accumulator
            0xF6 => {
                let data = self.read_immediate();
                self.ora(data);
            }

            // XRA Logical Exclusive-Or Register or Memory With Accumulator (Zero Accumulator)
            0xA8 => self.xra(self.register.b),
            0xA9 => self.xra(self.register.c),
            0xAA => self.xra(self.register.d),
            0xAB => self.xra(self.register.e),
            0xAC => self.xra(self.register.h),
            0xAD => self.xra(self.register.l),
            0xAE => self.xra(self.get_hl_data()),
            0xAF => self.xra(self.register.a),

            // XRI Exclusive-Or Immediate With Accumulator
            0xEE => {
                let data = self.read_immediate();
                self.xra(data);
            }

            // CMP Compare Register or Memory With Accumulator
            0xB8 => self.cmp(self.register.b),
            0xB9 => self.cmp(self.register.c),
            0xBA => self.cmp(self.register.d),
            0xBB => self.cmp(self.register.e),
            0xBC => self.cmp(self.register.h),
            0xBD => self.cmp(self.register.l),
            0xBE => self.cmp(self.get_hl_data()),
            0xBF => self.cmp(self.register.a),

            // CPI Compare Immediate With Accumulator
            0xFE => {
                let data = self.read_immediate();
                self.cmp(data);
            }

            // RLC Rotate Accumulator Left
            0x07 => self.rlc(),

            // RRC Rotate Accumulator Right
            0x0f => self.rrc(),

            // RAL Rotate Accumulator Left Through Carry
            0x17 => self.ral(),

            // RAR Rotate Accumulator Right Through Carry
            0x1F => self.rar(),

            // CMA Complement Accumulator
            0x2F => self.register.a = !self.register.a,

            // CMC Compliment Carry flag
            0x3F => self
                .register
                .set_flag(Flag::Carry, !self.register.get_flag(Flag::Carry)),

            // STC Set Carry
            0x37 => self.register.set_flag(Flag::Carry, true),

            // PCHL Load Program Counter
            0xE9 => self.register.program_counter = self.register.get_hl(),

            // Jump Instructions
            0xC3 | 0xDA | 0xD2 | 0xCA | 0xC2 | 0xFA | 0xF2 | 0xEA | 0xE2 => {
                let data = self.read_word_immediate();
                if let true = match opcode.0 {
                    // JMP Jump
                    0xC3 => true,
                    // JC Jump If
                    0xDA => self.register.get_flag(Flag::Carry),
                    // JNC Jump If Not Carry
                    0xD2 => !self.register.get_flag(Flag::Carry),
                    // JZ Jump If Zero
                    0xCA => self.register.get_flag(Flag::Zero),
                    // JNZ Jump If Not Zero
                    0xC2 => !self.register.get_flag(Flag::Zero),
                    // JM Jump If Mimus
                    0xFA => self.register.get_flag(Flag::Sign),
                    // JP Jump If Positive
                    0xF2 => !self.register.get_flag(Flag::Sign),
                    // JPE Jump If Parity Even
                    0xEA => self.register.get_flag(Flag::Parity),
                    // JPO Jump If Parity Odd
                    0xE2 => !self.register.get_flag(Flag::Parity),
                    _ => unimplemented!(),
                } {
                    self.register.program_counter = data;
                    // return opcode.cycles();
                }
            }

            // Call Subroutine Instructions
            0xCD | 0xDC | 0xD4 | 0xCC | 0xC4 | 0xFC | 0xF4 | 0xEC | 0xE4 => {
                let data = self.read_word_immediate();
                if let true = match opcode.0 {
                    // CALL Call
                    0xCD => true,
                    // CC Call If Carry
                    0xDC => self.register.get_flag(Flag::Carry),
                    // CNC Call If No Carry
                    0xD4 => !self.register.get_flag(Flag::Carry),
                    // CZ Call If Zero
                    0xCC => self.register.get_flag(Flag::Zero),
                    // CNZ Call If Not Zero
                    0xC4 => !self.register.get_flag(Flag::Zero),
                    // CM Call If Minus
                    0xFC => self.register.get_flag(Flag::Sign),
                    // CP Call If Plus
                    0xF4 => !self.register.get_flag(Flag::Sign),
                    // CPE Call If Parity Even
                    0xEC => self.register.get_flag(Flag::Parity),
                    // CPO Call If Parity Odd
                    0xE4 => !self.register.get_flag(Flag::Parity),
                    _ => unimplemented!(),
                } {
                    self.push(self.register.program_counter);
                    self.register.program_counter = data;
                    if opcode.0 != 0xCD {
                        cycle += 6;
                    }
                    // return opcode.cycles();
                }
            }

            // Return From Subroutine Instructions
            0xC9 | 0xD8 | 0xD0 | 0xC8 | 0xC0 | 0xF8 | 0xF0 | 0xE8 | 0xE0 => {
                if let true = match opcode.0 {
                    // RET Return
                    0xC9 => true,
                    // RC Return If Carry
                    0xD8 => self.register.get_flag(Flag::Carry),
                    // RNC Return If No Carry
                    0xD0 => !self.register.get_flag(Flag::Carry),
                    // RZ Return If Zero
                    0xC8 => self.register.get_flag(Flag::Zero),
                    // RNZ Return If Not Zero
                    0xC0 => !self.register.get_flag(Flag::Zero),
                    // RM Return If Minus
                    0xF8 => self.register.get_flag(Flag::Sign),
                    // RP Return If Plus
                    0xF0 => !self.register.get_flag(Flag::Sign),
                    // RPE Return If Parity Even
                    0xE8 => self.register.get_flag(Flag::Parity),
                    // RPO Return If Parity Odd
                    0xE0 => !self.register.get_flag(Flag::Parity),
                    _ => unimplemented!(),
                } {
                    self.register.program_counter = self.pop();
                    // cycle += 6;
                    if opcode.0 != 0xc9 {
                        cycle += 6;
                    }
                    // return opcode.cycles();
                }
            }

            // RST Instructions
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                self.push(self.register.program_counter);
                self.register.program_counter = (opcode.0 & 0x38) as u16;
            }

            // PUSH Push Data Onto Stack
            0xC5 => self.push(self.register.get_bc()),
            0xD5 => self.push(self.register.get_de()),
            0xE5 => self.push(self.register.get_hl()),
            0xF5 => self.push(self.register.get_af()),

            // POP Pop Data Off Stack
            0xC1 => {
                let data = self.pop();
                self.register.set_bc(data);
            }
            0xD1 => {
                let data = self.pop();
                self.register.set_de(data);
            }
            0xE1 => {
                let data = self.pop();
                self.register.set_hl(data);
            }
            0xF1 => {
                let data = self.pop();
                self.register.set_af(data);
            }

            // XTHL Exchange Stack
            0xE3 => {
                let hl = self.register.get_hl();
                let stack_pointer = self.memory.read_word(self.register.stack_pointer);
                self.register.set_hl(stack_pointer);
                self.memory.write_word(self.register.stack_pointer, hl);
            }

            // SPHL Load SP From Hand L
            0xF9 => self.register.stack_pointer = self.register.get_hl(),

            // IN Input
            0xDB => {
                let port = self.read_immediate();
                self.register.a = machine.input(port);
            }

            // OUT Output
            0xD3 => {
                let port = self.read_immediate();
                machine.output(port, self.register.a);
            }

            // EI Enable Interrupts
            0xFB => self.interrupt_enabled = true,

            // DI Disable Interrupts
            0xF3 => self.interrupt_enabled = false,

            // HLT Halt Instruction
            0x76 => self.halted = true,

            _ => unimplemented!(),
        }

        cycle
    }

    fn add(&mut self, other: u8) {
        let sum = self.register.a.wrapping_add(other);
        self.register.set_flag(Flag::Zero, sum == 0);
        self.register.set_flag(Flag::Sign, sum & 0x80 != 0);
        self.register.set_flag(
            Flag::AuxiliaryCarry,
            (self.register.a & 0x0F) + (other & 0x0F) > 0x0F,
        );
        self.register
            .set_flag(Flag::Parity, sum.count_ones() & 0x01 == 0);
        self.register
            .set_flag(Flag::Carry, self.register.a as u16 + other as u16 > 0xFF);
        self.register.a = sum;
    }

    fn adc(&mut self, other: u8) {
        let carry = self.register.get_flag(Flag::Carry) as u8;
        let sum = self.register.a.wrapping_add(other).wrapping_add(carry);
        self.register.set_flag(Flag::Zero, sum == 0);
        self.register.set_flag(Flag::Sign, sum & 0x80 != 0);
        self.register.set_flag(
            Flag::AuxiliaryCarry,
            (self.register.a & 0x0F) + (other & 0x0F) + carry > 0x0F,
        );
        self.register
            .set_flag(Flag::Parity, sum.count_ones() & 0x01 == 0);
        self.register.set_flag(
            Flag::Carry,
            self.register.a as u16 + other as u16 + carry as u16 > 0xFF,
        );
        self.register.a = sum;
    }

    fn sub(&mut self, other: u8) {
        let diff = self.register.a.wrapping_sub(other);
        self.register.set_flag(Flag::Zero, diff == 0);
        self.register.set_flag(Flag::Sign, diff & 0x80 != 0);
        self.register.set_flag(
            Flag::AuxiliaryCarry,
            (self.register.a & 0x0F).wrapping_sub(other & 0x0F) > 0x0F,
        );
        self.register
            .set_flag(Flag::Parity, diff.count_ones() & 0x01 == 0);
        self.register.set_flag(Flag::Carry, self.register.a < other);
        self.register.a = diff;
    }

    fn sbb(&mut self, other: u8) {
        let carry = self.register.get_flag(Flag::Carry) as u8;
        let diff = self.register.a.wrapping_sub(other).wrapping_sub(carry);
        self.register.set_flag(Flag::Zero, diff == 0);
        self.register.set_flag(Flag::Sign, diff & 0x80 != 0);
        self.register.set_flag(
            Flag::AuxiliaryCarry,
            (self.register.a & 0x0F)
                .wrapping_sub(other & 0x0F)
                .wrapping_sub(carry)
                > 0x0F,
        );
        self.register
            .set_flag(Flag::Parity, diff.count_ones() & 0x01 == 0);
        self.register.set_flag(
            Flag::Carry,
            (self.register.a as u16) < (other as u16 + carry as u16),
        );
        self.register.a = diff;
    }

    fn inr(&mut self, other: u8) -> u8 {
        let sum = other.wrapping_add(1);
        self.register.set_flag(Flag::Zero, sum == 0);
        self.register.set_flag(Flag::Sign, sum & 0x80 != 0);
        self.register
            .set_flag(Flag::AuxiliaryCarry, (other & 0x0F) + 1 > 0x0F);
        self.register
            .set_flag(Flag::Parity, sum.count_ones() & 0x01 == 0);
        sum
    }

    fn dcr(&mut self, other: u8) -> u8 {
        let diff = other.wrapping_sub(1);
        self.register.set_flag(Flag::Zero, diff == 0);
        self.register.set_flag(Flag::Sign, diff & 0x80 != 0);
        self.register
            .set_flag(Flag::AuxiliaryCarry, (other & 0x0F).wrapping_sub(1) > 0x0F);
        self.register
            .set_flag(Flag::Parity, diff.count_ones() & 0x01 == 0);
        diff
    }

    fn dad(&mut self, other: u16) {
        let hl = self.register.get_hl();
        let sum = hl.wrapping_add(other);
        self.register.set_flag(Flag::Carry, hl > 0xFFFF - other);
        self.register.set_hl(sum);
    }

    fn daa(&mut self) {
        let low_bytes = self.register.a & 0x0F;
        let high_bytes  = self.register.a >> 4;
        let mut increment = 0;

        if low_bytes > 9 || self.register.get_flag(Flag::AuxiliaryCarry) {
            increment += 0x06;
        }

        if high_bytes > 9 || self.register.get_flag(Flag::Carry) {
            increment += 0x60;
        }

        self.add(increment);
    }

    fn ana(&mut self, other: u8) {
        let result = self.register.a & other;
        self.register.set_flag(Flag::Zero, result == 0);
        self.register.set_flag(Flag::Sign, result & 0x80 == 0x80);
        self.register.set_flag(Flag::AuxiliaryCarry, result > 0x0F);
        self.register
            .set_flag(Flag::Parity, result.count_ones() & 0x01 == 0);
        self.register.set_flag(Flag::Carry, false);
        self.register.a = result;
    }

    fn ora(&mut self, other: u8) {
        let result = self.register.a | other;
        self.register.set_flag(Flag::Zero, result == 0);
        self.register.set_flag(Flag::Sign, result & 0x80 == 0x80);
        self.register.set_flag(Flag::AuxiliaryCarry, false);
        self.register
            .set_flag(Flag::Parity, result.count_ones() & 0x01 == 0);
        self.register.set_flag(Flag::Carry, false);
        self.register.a = result;
    }

    fn xra(&mut self, other: u8) {
        let result = self.register.a ^ other;
        self.register.set_flag(Flag::Zero, result == 0);
        self.register.set_flag(Flag::Sign, result & 0x80 == 0x80);
        self.register.set_flag(Flag::AuxiliaryCarry, false);
        self.register
            .set_flag(Flag::Parity, result.count_ones() & 0x01 == 0);
        self.register.set_flag(Flag::Carry, false);
        self.register.a = result;
    }

    fn cmp(&mut self, other: u8) {
        let a = self.register.a;
        self.sub(other);
        self.register.a = a;
    }

    fn rlc(&mut self) {
        let bit7 = self.register.a & 0x80;
        self.register.set_flag(Flag::Carry, bit7 == 0x80);
        self.register.a = (self.register.a << 1) | bit7;
    }

    fn rrc(&mut self) {
        let bit0 = self.register.a & 0x01;
        self.register.set_flag(Flag::Carry, bit0 == 0x01);
        self.register.a = (self.register.a >> 1) | (bit0 << 7);
    }

    fn ral(&mut self) {
        let carry = self.register.get_flag(Flag::Carry) as u8;
        let bit7 = self.register.a & 0x80;
        self.register.set_flag(Flag::Carry, bit7 == 0x80);
        self.register.a = (self.register.a << 1) | carry;
    }

    fn rar(&mut self) {
        let carry = self.register.get_flag(Flag::Carry) as u8;
        let bit0 = self.register.a & 0x01;
        self.register.set_flag(Flag::Carry, bit0 == 0x01);
        self.register.a = (self.register.a >> 1) | (carry << 7);
    }

    fn push(&mut self, other: u16) {
        self.register.stack_pointer = self.register.stack_pointer.wrapping_sub(2);
        self.memory.write_word(self.register.stack_pointer, other);
    }

    fn pop(&mut self) -> u16 {
        let data = self.memory.read_word(self.register.stack_pointer);
        self.register.stack_pointer = self.register.stack_pointer.wrapping_add(2);
        data
    }

    pub fn interrupt(&mut self, interrupt_number: u16) {
        if self.interrupt_enabled {
            self.push(self.register.program_counter);
            self.register.program_counter = 8 * interrupt_number;
            self.interrupt_enabled = false;
        }
    }

    pub fn set_hl_data(&mut self, data: u8) {
        self.memory.write(self.register.get_hl(), data);
    }

    pub fn get_hl_data(&self) -> u8 {
        let address = self.register.get_hl();
        self.memory.read(address)
    }

    pub fn read_immediate(&mut self) -> u8 {
        let data = self.memory.read(self.register.program_counter);
        self.register.program_counter += 1;
        data
    }

    pub fn read_word_immediate(&mut self) -> u16 {
        let data = self.memory.read_word(self.register.program_counter);
        self.register.program_counter += 2;
        data
    }

    pub fn load_rom(&mut self, memory: &[u8], position: u16) {
        self.memory.load(memory, position);
        self.register.program_counter = position
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register: Register::default(),
            memory: Memory::default(),
            interrupt_enabled: false,
            halted: false,
            show_debug_log: true,
        }
    }
}
