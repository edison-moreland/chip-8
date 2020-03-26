use super::instructions::Instruction;
use super::Chip8Error;
use super::traits::Drawable;
use std::convert::TryInto;

use std::cell::RefCell;
use std::rc::Rc;

// Font built in to Chip-8. 16 characters(0-F), 5 bytes each
// for a grand total of 80 bytes. This should be loaded into
// the first 512 bytes of program memory before execution starts.
pub const CHIP8_FONT: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub fn character_offset(character: u16) -> u16 {
    (character as u16) * 5
}

// Address where fonts will be loaded to
const FONT_START: usize = 0x000;

// Chip-8 programs get loaded into memory starting at 0x200
// everything below that is reserved for the system.
const PROGRAM_START: usize = 0x200;

const MEM_SIZE: usize = 0xFFF + 1;
const V_REG_SIZE: usize = 0xF + 1;
const STACK_SIZE: usize = 0xF + 1;

pub struct Chip8 {
    mem: [u8; MEM_SIZE],

    // Registers
    v_reg: [u8; V_REG_SIZE],
    i_reg: u16,
    delay_reg: u8,
    sound_reg: u8,

    program_counter: u16,
    stack: Vec<u16>,

    screen: Box<dyn Drawable>,
}

impl Chip8 {
    pub fn new(screen: Box<dyn Drawable>) -> Self {
        Self {
            mem: [0; MEM_SIZE],

            // Registers
            v_reg: [0; V_REG_SIZE],
            i_reg: 0,
            delay_reg: 0,
            sound_reg: 0,

            program_counter: PROGRAM_START as u16,
            stack: Vec::with_capacity(STACK_SIZE),

            screen: screen,
        }
    }

    pub fn init_memory(&mut self, rom: &[u8]) -> Result<(), Chip8Error> {
        self.load_rom(FONT_START, &CHIP8_FONT)?;

        self.load_rom(PROGRAM_START, rom)?;

        return Ok(());
    }

    pub fn step_execution(&mut self) -> Result<(), Chip8Error> {
        // todo: do some timer stuff

        let instruction = self.next_instruction()?;
        self.execute_instruction(instruction)?;

        return Ok(());
    }

    fn next_instruction(&mut self) -> Result<Instruction, Chip8Error> {
        let pc = self.program_counter as usize;

        let bytes: [u8; 2] = self.mem[pc..pc + 2].try_into().unwrap();
        self.program_counter += 2;

        return match Instruction::from_bytes(bytes) {
            Ok(inst) => Ok(inst),
            Err(_) => Err(Chip8Error::InvalidInstruction(u16::from_be_bytes(bytes), self.program_counter-2))
        }
    }

    fn load_rom(&mut self, start_address: usize, rom: &[u8]) -> Result<(), Chip8Error> {
        let end_address = start_address + rom.len();
        if end_address >= MEM_SIZE {
            return Err(Chip8Error::RomTooBig(rom.len()));
        }

        self.mem[start_address..end_address].clone_from_slice(rom);

        return Ok(());
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
        match instruction {
            // Control Flow
            Instruction::Jump(addr) => {
                self.program_counter = addr;
                Ok(())
            }
            Instruction::SkipIfEqualImm(vx, imm) => {
                if self.v_reg[vx as usize] == imm {
                    self.program_counter += 2
                }

                Ok(())
            }
            Instruction::SkipIfNotEqualImm(vx, imm) => {
                if self.v_reg[vx as usize] != imm {
                    self.program_counter += 2
                }

                Ok(())
            }
            Instruction::SkipIfEqualReg(vx, vy) => {
                if self.v_reg[vx as usize] == self.v_reg[vy as usize] {
                    self.program_counter += 2
                }

                Ok(())
            }
            Instruction::SkipIfNotEqualReg(vx, vy) => {
                if self.v_reg[vx as usize] != self.v_reg[vy as usize] {
                    self.program_counter += 2
                }

                Ok(())
            }
            Instruction::SubroutineCall(addr) => {
                self.stack.push(self.program_counter);
                
                self.program_counter = addr;

                Ok(())
            }
            Instruction::SubroutineReturn() => {
                // If stack underflows, program returns to the start
                // haven't seen any documentation on what is actually supposed to happen
                self.program_counter = self.stack.pop().unwrap_or(PROGRAM_START as u16);

                Ok(())
            }

            // Memory
            Instruction::LoadRegReg(vx, vy) => {
                self.v_reg[vx as usize] = self.v_reg[vy as usize];

                Ok(())
            }
            Instruction::LoadRegImm(vx, imm) => {
                self.v_reg[vx as usize] = imm;
                Ok(())
            }
            Instruction::LoadAddress(addr) => {
                self.i_reg = addr;
                Ok(())
            }
            Instruction::LoadMemoryRegisters(vx) => {
                // Dumps registers v0..vx to memory starting at i
                let regs = &self.v_reg[..vx as usize];

                let reg_start = self.i_reg as usize;
                let reg_end = reg_start + (vx as usize);
                self.mem[reg_start..reg_end].clone_from_slice(regs);

                Ok(())
            }
            Instruction::LoadRegistersMemory(vx) => {
                // Loads registers v0..vx from memory starting at i
                let reg_start = self.i_reg as usize;
                let reg_end = reg_start + (vx as usize);
                let regs = &self.mem[reg_start..reg_end];
                
                self.v_reg[..vx as usize].clone_from_slice(regs);

                Ok(())
            }

            // Display
            Instruction::Draw(vx, vy, sprite_size) => {
                let x = self.v_reg[vx as usize] as usize;
                let y = self.v_reg[vy as usize] as usize;

                let sprite_start = self.i_reg as usize;
                let sprite_end = sprite_start + (sprite_size as usize);
                let sprite = &self.mem[sprite_start..sprite_end];
                
                let did_collide = self.screen.write_sprite(x, y, sprite);
                self.v_reg[0xF] = did_collide as u8;

                self.screen.flush();

                Ok(())
            }
            
            // Math
            Instruction::AddImm(vx, imm) => {
                let (new_val, did_overflow) = self.v_reg[vx as usize].overflowing_add(imm);
                
                self.v_reg[vx as usize] = new_val;
                self.v_reg[0xF] = did_overflow as u8;

                Ok(())
            }
            Instruction::AddReg(vx, vy) => {
                let (new_val, did_overflow) = self.v_reg[vx as usize].overflowing_add(self.v_reg[vy as usize]);
                
                self.v_reg[vx as usize] = new_val;
                self.v_reg[0xF] = did_overflow as u8;

                Ok(())
            }
            Instruction::SubtractReg(vx, vy) => {
                let x = self.v_reg[vx as usize];
                let y = self.v_reg[vy as usize];

                self.v_reg[0xF] = (y >= x) as u8;
                
                self.v_reg[vx as usize] = x.wrapping_sub(y);

                Ok(())
            }
            
            // Logic
            Instruction::OrReg(vx, vy) => {
                self.v_reg[vx as usize] |= self.v_reg[vy as usize];

                Ok(())
            }
            Instruction::AndReg(vx, vy) => {
                self.v_reg[vx as usize] &= self.v_reg[vy as usize];

                Ok(())
            }
            Instruction::XorReg(vx, vy) => {
                self.v_reg[vx as usize] ^= self.v_reg[vy as usize];

                Ok(())
            }
            Instruction::ShiftLeft(vx, _) => {
                let x = self.v_reg[vx as usize];

                self.v_reg[0xF] = x & 0b0000_0001;

                self.v_reg[vx as usize] = x.wrapping_shl(1);

                Ok(())
            }
            Instruction::ShiftRight(vx, _) => {
                let x = self.v_reg[vx as usize];

                self.v_reg[0xF] = (x & 0b1000_0000) >> 7;

                self.v_reg[vx as usize] = x.wrapping_shr(1);

                Ok(())
            }
            _ => Err(Chip8Error::InstructionNotImplemented(instruction))
        }
    }
}
