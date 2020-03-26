mod chip8;
pub use self::chip8::*;

mod instructions;
use self::instructions::Instruction;

use std::fmt;

pub enum Chip8Error {
    RomTooBig(usize),
    InvalidInstruction(u16),
    InstructionNotImplemented(Instruction),
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chip8Error::InvalidInstruction(inst) => write!(f, "InvalidInstruction {:#04X}", inst),
            Chip8Error::RomTooBig(size) => write!(f, "RomTooBig {} bytes", size),
            Chip8Error::InstructionNotImplemented(inst) => {
                write!(f, "InstructionNotImplemented {}", inst)
            }
        }
    }
}
