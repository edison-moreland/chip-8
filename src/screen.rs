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
pub fn character_offset(character: u16) -> u16 {
    (character as u16) * 5
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

        // return true if sprite erases any pixels
        let mut did_collide = false;

        // I believe this function could be vectorized once the WASM SIMD
        // spec makes it down the pipeline, but for now it doesn't matter 
        for (i, line) in sprite.iter().enumerate() {
            // Chip-8 limitation, sprites are only 15 bytes long
            if i>15 {break}

            // Prep line to be XORd to the screen. 
            // Reverse bits so sprite isn't backwards (not sure why this happens)
            // Shift into x position, truncating bits that go off the edge
            let reversed = ((*line).reverse_bits() as u64).wrapping_shl(x as u32);

            // Check if XOR will erase any pixels
            let xord = self.raw[i+y] ^ reversed;
            did_collide |= (self.raw[i+y] & reversed) != 0;

            self.raw[i+y] = xord
        }
        return did_collide;
    }
} 