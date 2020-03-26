use super::instructions::Instruction;
use super::Chip8Error;
use std::convert::TryInto;

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
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            mem: [0; MEM_SIZE],

            // Registers
            v_reg: [0; V_REG_SIZE],
            i_reg: 0,
            delay_reg: 0,
            sound_reg: 0,

            program_counter: PROGRAM_START as u16,
            stack: Vec::with_capacity(STACK_SIZE),
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

        let bytes = &self.mem[pc..pc + 2];
        self.program_counter += 2;

        return Ok(Instruction::from_bytes(bytes.try_into().unwrap())?);
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
            Instruction::Jump(addr) => {
                self.program_counter = addr;
                Ok(())
            }
            _ => Err(Chip8Error::InstructionNotImplemented(instruction)),
        }
    }
}
