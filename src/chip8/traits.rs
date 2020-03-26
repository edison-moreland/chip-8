
pub trait Drawable {
    fn write_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool;

    fn flush(&self);
}