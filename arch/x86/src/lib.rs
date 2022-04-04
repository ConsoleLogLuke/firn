pub mod cpu;
pub mod device;
pub mod flags;
pub mod instr;
pub mod modrm;
pub mod opcodes;
pub mod regs;
pub mod system;

pub use cpu::Cpu;
pub use flags::Flags;
pub use instr::{Instr, InstrFunc};
pub use modrm::{Displacement, Modrm, ModrmRegType, RegMem, RmPtr};
pub use regs::{GeneralByteReg, GeneralReg, GeneralWordReg, Reg, SegmentReg, WordReg};
pub use system::ExtSystem;

pub const DEFAULT_BIOS: &[u8] = include_bytes!("../resources/default_bios.bin");

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Byte,
    Word,
}
