const MEM_SIZE: usize = 0xFFF+1;
const V_REG_SIZE: usize = 0xF+1;
const STACK_SIZE: usize = 0xF+1;

pub struct Chip8 {
    mem: [u8; MEM_SIZE],
    
    // Registers
    v_reg: [u8; V_REG_SIZE],
    i_reg: u16,
    delay_reg: u8,
    sound_reg: u8,

    program_counter: u16,
    stack: Vec::<u16>,
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
        
            program_counter: 0,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }
}