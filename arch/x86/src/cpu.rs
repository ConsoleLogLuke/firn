use crate::GeneralWordReg::Sp;
use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{Flags, GeneralByteReg, Instr, SegmentReg, WordReg};
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

    pub fn mem_8(&self, segment: SegmentReg, offset: u16) -> u8 {
        let linear = self.linear_mem(segment, offset);

        self.system.mem[linear]
    }

    pub fn mem_16(&self, segment: SegmentReg, offset: u16) -> u16 {
        let linear = self.linear_mem(segment, offset);

        let low = self.system.mem[linear];
        let high = self.system.mem[linear + 1];

        u16::from_le_bytes([low, high])
    }

    pub fn set_mem_8(&mut self, segment: SegmentReg, offset: u16, value: u8) {
        let linear = self.linear_mem(segment, offset);
        self.system.mem[linear] = value;
    }

    pub fn set_mem_16(&mut self, segment: SegmentReg, offset: u16, value: u16) {
        let linear = self.linear_mem(segment, offset);

        let [low, high] = value.to_le_bytes();
        self.system.mem[linear] = low;
        self.system.mem[linear + 1] = high;
    }

    pub fn read_mem_8(&mut self) -> u8 {
        let value = self.mem_8(Cs, self.ip);
        self.ip += 1;

        value
    }

    pub fn read_mem_16(&mut self) -> u16 {
        let value = self.mem_16(Cs, self.ip);
        self.ip += 2;

        value
    }

    pub fn push_8(&mut self, value: u8) {
        let sp = self.reg_16(Sp.into()) - 1;
        self.set_reg_16(Sp.into(), sp);
        self.set_mem_8(Ss, sp, value);
    }

    pub fn push_16(&mut self, value: u16) {
        let sp = self.reg_16(Sp.into()) - 2;
        self.set_reg_16(Sp.into(), sp);
        self.set_mem_16(Ss, sp, value);
    }

    pub fn pop_8(&mut self) -> u8 {
        let sp = self.reg_16(Sp.into());
        let value = self.mem_8(Ss, sp);
        self.inc_reg_16(Sp.into(), 1);

        value
    }

    pub fn pop_16(&mut self) -> u16 {
        let sp = self.reg_16(Sp.into());
        let value = self.mem_16(Ss, sp);
        self.inc_reg_16(Sp.into(), 2);

        value
    }

    pub fn push_reg_8(&mut self, reg: GeneralByteReg) {
        let value = self.reg_8(reg);
        self.push_8(value);
    }

    pub fn push_reg_16(&mut self, reg: WordReg) {
        let value = self.reg_16(reg);
        self.push_16(value);
    }

    fn linear_mem(&self, segment: SegmentReg, offset: u16) -> usize {
        let segment = self.reg_16(segment.into()) as usize;

        (segment << 4) + offset as usize
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
        print!("({:#x}) ", sys.cpu.linear_mem(Cs, sys.cpu.ip));
        io::stdout().flush().unwrap();

        let instr = Instr::decode(&mut sys.cpu);
        sys.cpu.decoded += 1;
        println!("({:02}) Decoded: {:?}", sys.cpu.decoded, instr);
        instr.execute(&mut sys.cpu);
    }
}
