use bit_reverse::ParallelReverse;

// Font built in to Chip-8. 16 characters(0-F), 5 bytes each 
// for a grand total of 80 bytes. This should be loaded into 
// the first 512 bytes of program memory before execution starts.
pub const CHIP8_FONT: [u8; 5*16] = [
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

// Maybe this should be a macro?
pub fn character_offset(character: u8) -> usize {
    (character as usize) * 5
}

pub struct Screen{
    raw: [u64; 32]
}

impl Screen {
    pub fn new_empty() -> Screen {
        Screen{ raw: [0; 32] }
    }

    pub fn as_raw(&self) -> [u64; 32] {
        self.raw
    }

    pub fn write_sprite(&mut self, mut x: usize, mut y: usize, sprite: &[u8]) -> bool {
        // Not sure if this is the standard way to handle drawing off screen.
        // The only reference I've seen to it is in Mikolay's Chip-8 reference.
        if x > 0x3F { x %= 64 }
        if y > 0x1F { y %= 32 }

        // returns true if sprite erased any pixels
        for (i, line) in sprite.iter().enumerate() {
            // Not sure why the reverse is needed yet
            // Something to do with converting to u64
            let reversed = (*line).swap_bits() as u64;
            self.raw[i+y] ^= reversed << x;
            // Chip-8 sprites can only be 15 bytes long
            // truncate if it goes over
        }
        return false;
    } 
}