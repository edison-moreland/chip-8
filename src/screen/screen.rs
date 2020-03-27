use super::canvas::Canvas;
use crate::chip8::traits::Drawable;

pub struct Screen {
    raw: [u64; 32],
    canvas: Canvas,
}

impl Screen {
    pub fn new_empty(canvas: Canvas) -> Screen {
        Screen {
            raw: [0; 32],
            canvas: canvas,
        }
    }
}

impl Drawable for Screen {
    fn flush(&self) {
        self.canvas.draw_grid(&self.raw)
    }

    fn clear(&mut self) {
        self.raw = [0; 32];
    }

    fn write_sprite(&mut self, mut x: usize, mut y: usize, sprite: &[u8]) -> bool {
        // Not sure if this is the standard way to handle drawing off screen.
        // The only reference I've seen to it is in Mikolay's Chip-8 reference.
        if x > 0x3F {
            x %= 64
        }
        if y > 0x1F {
            y %= 32
        }

        // return true if sprite erases any pixels
        let mut did_collide = false;

        // I believe this function could be vectorized once the WASM SIMD
        // spec makes it down the pipeline, but for now it doesn't matter
        for (i, line) in sprite.iter().enumerate() {
            // Chip-8 limitation, sprites are only 15 bytes long
            if i > 15 {
                break;
            }

            // Prep line to be XORd to the screen.
            // Reverse bits so sprite isn't backwards (not sure why this happens)
            // Shift into x position, truncating bits that go off the edge
            let reversed = ((*line).reverse_bits() as u64).wrapping_shl(x as u32);

            // Check if XOR will erase any pixels
            let xord = self.raw[i + y] ^ reversed;
            did_collide |= (self.raw[i + y] & reversed) != 0;

            self.raw[i + y] = xord
        }
        return did_collide;
    }
}
