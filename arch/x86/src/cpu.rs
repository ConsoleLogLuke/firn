use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{ExtSystem, Flags, GeneralByteReg, Instr, WordReg};
use firn_core::{cpu, System};
use std::io;
use std::io::Write;

pub struct Cpu {
    regs: [u8; 2 * 8],
    segments: [u16; 4],
    pub flags: Flags,
    pub ip: u16,

    pub decoded: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: [0; 2 * 8],
            segments: [0; 4],
            flags: Flags::new(),
            ip: 0,

            decoded: 0,
        }
    }

    pub fn reg_8(&self, reg: GeneralByteReg) -> u8 {
        self.regs[reg as usize]
    }

    pub fn reg_16(&self, reg: WordReg) -> u16 {
        match reg {
            WordReg::General(reg) => {
                let low = self.regs[reg as usize];
                let high = self.regs[reg as usize + 4];

                u16::from_le_bytes([low, high])
            }
            WordReg::Segment(reg) => self.segments[reg as usize],
        }
    }

    pub fn set_reg_8(&mut self, reg: GeneralByteReg, value: u8) {
        self.regs[reg as usize] = value;
    }

    pub fn set_reg_16(&mut self, reg: WordReg, value: u16) {
        match reg {
            WordReg::General(reg) => {
                let [low, high] = value.to_le_bytes();

                self.regs[reg as usize] = low;
                self.regs[reg as usize + 4] = high;
            }
            WordReg::Segment(reg) => self.segments[reg as usize] = value,
        };
    }

    pub fn inc_reg_8(&mut self, reg: GeneralByteReg, amount: u8) {
        let old = self.reg_8(reg);
        self.set_reg_8(reg, old.wrapping_add(amount));
    }

    pub fn inc_reg_16(&mut self, reg: WordReg, amount: u16) {
        let old = self.reg_16(reg);
        self.set_reg_16(reg, old.wrapping_add(amount));
    }

    pub fn dec_reg_8(&mut self, reg: GeneralByteReg, amount: u8) {
        let old = self.reg_8(reg);
        self.set_reg_8(reg, old.wrapping_sub(amount));
    }

    pub fn dec_reg_16(&mut self, reg: WordReg, amount: u16) {
        let old = self.reg_16(reg);
        self.set_reg_16(reg, old.wrapping_sub(amount));
    }
}

impl cpu::Cpu for Cpu {
    fn reset(&mut self) {
        self.set_reg_16(Cs.into(), 0xffff);
        self.set_reg_16(Ds.into(), 0x0000);
        self.set_reg_16(Es.into(), 0x0000);
        self.set_reg_16(Ss.into(), 0x0000);

        self.ip = 0;
    }

    fn step(sys: &mut System<Self>) {
        print!("({:#x}) ", sys.linear_mem(Cs, sys.cpu.ip));
        io::stdout().flush().unwrap();

        let instr = Instr::decode(&mut sys.cpu);
        sys.cpu.decoded += 1;
        println!("({:02}) Decoded: {:?}", sys.cpu.decoded, instr);
        instr.execute(&mut sys.cpu);
    }
}