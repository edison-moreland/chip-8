use std::fmt;

fn first_nibble(x: u8) -> u8 {
    return (x & 0xF0) >> 4;
}

fn last_nibble(x: u8) -> u8 {
    return x & 0x0F;
}

fn address(x: [u8; 2]) -> u16 {
    return u16::from_be_bytes(x) & 0x0FFF;
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Clears the screen
    ClearScreen(),

    // Return from a subroutine
    SubroutineReturn(),

    // (addr) Jump to address at addr
    Jump(u16),

    // (addr) Call subroutine at addr
    SubroutineCall(u16),

    // (Vx, kk) Skip next instruction if Vx == kk
    SkipIfEqualImm(u8, u8),

    // (Vx, kk) Skip next instruction if Vx != kk
    SkipIfNotEqualImm(u8, u8),

    // (Vx, Vy) Skip next instruction if Vx == Vy
    SkipIfEqualReg(u8, u8),

    // (Vx, kk) Load Vx with byte kk
    LoadRegImm(u8, u8),

    // (Vx, kk) Add kk to Vx
    AddImm(u8, u8),

    // (Vx, Vy) Load Vx with Vy
    LoadRegReg(u8, u8),

    // (Vx, Vy) Bitwise OR Vx with Vy
    OrReg(u8, u8),

    // (Vx, Vy) Bitwise AND Vx with Vy
    AndReg(u8, u8),

    // (Vx, Vy) Bitwise XOR Vx with Vy
    XorReg(u8, u8),

    // (Vx, Vy) Add Vy to Vx, set VF to carry
    AddReg(u8, u8),

    // (Vx, Vy) Subtract Vy from Vx, set VF to NOT borrow
    SubtractReg(u8, u8),

    // (Vx, Vy) Shift Vx right 1
    ShiftRight(u8, u8),

    // (Vx, Vy) Set Vx to Vy - Vx
    SubtractRegSwapped(u8, u8),

    // (Vx, Vy) Shift Vx left 1
    ShiftLeft(u8, u8),

    // (Vx, Vy) Skip next instruction if Vx != Vy
    SkipIfNotEqualReg(u8, u8),

    // (addr) Load addr into i
    LoadAddress(u16),

    // (addr) Jump to addr + V0
    JumpV0(u16),

    // (Vx, kk) Set Vx to kk AND random number
    Random(u8, u8),

    // (Vx, Vy, n) Draw sprite
    Draw(u8, u8, u8),

    // (Vx) Skip next instruction if key in Vx pressed
    SkipIfPressed(u8),

    // (Vx) Skip next instruction if key in Vx not pressed
    SkipIfNotPressed(u8),

    // (Vx) Load Vx with value of delay timer
    LoadRegDelay(u8),

    // (Vx) Wait for key press, set Vx to value of key
    LoadKey(u8),

    // (Vx) Load delay timer with value of Vx
    LoadDelayReg(u8),

    // (Vx) Load sound timer with value of Vx
    LoadSoundReg(u8),

    // (Vx) Add Vx to I
    AddAddress(u8),

    // (Vx) Set I to location of sprite for digit in Vx
    LoadAddressDigit(u8),

    // (Vx) Store BCD of Vx at address in I
    LoadMemoryBcd(u8),

    // (Vx) Store registers V0-Vx in memory starting at address I
    LoadMemoryRegisters(u8),

    // (Vx) Read registers V0-Vx from memory starting at address I
    LoadRegistersMemory(u8),
}

impl Instruction {
    pub fn from_bytes(raw: [u8; 2]) -> Result<Instruction, ()> {
        // Instructions have arguments in a few standard places
        // 0nnn - 12 bit address
        let nnn = address(raw);

        // 0x00 - 4 bit, usually the first register
        let x = last_nibble(raw[0]);

        // 00y0 - 4 bit, usually the second register
        let y = first_nibble(raw[1]);

        // 000n - 4 bit
        let n = last_nibble(raw[1]);

        // 00kk - 8 bit
        let kk = raw[1];

        return match first_nibble(raw[0]) {
            0x0 => match raw[1] {
                0xE0 => Ok(Instruction::ClearScreen()),
                0xEE => Ok(Instruction::SubroutineReturn()),
                _ => Err(()),
            },
            0x1 => Ok(Instruction::Jump(nnn)),
            0x2 => Ok(Instruction::SubroutineCall(nnn)),
            0x3 => Ok(Instruction::SkipIfEqualImm(x, kk)),
            0x4 => Ok(Instruction::SkipIfNotEqualImm(x, kk)),
            0x5 => Ok(Instruction::SkipIfEqualReg(x, y)),
            0x6 => Ok(Instruction::LoadRegImm(x, kk)),
            0x7 => Ok(Instruction::AddImm(x, kk)),
            0x8 => match last_nibble(raw[1]) {
                0x0 => Ok(Instruction::LoadRegReg(x, y)),
                0x1 => Ok(Instruction::OrReg(x, y)),
                0x2 => Ok(Instruction::AndReg(x, y)),
                0x3 => Ok(Instruction::XorReg(x, y)),
                0x4 => Ok(Instruction::AddReg(x, y)),
                0x5 => Ok(Instruction::SubtractReg(x, y)),
                0x6 => Ok(Instruction::ShiftRight(x, y)),
                0x7 => Ok(Instruction::SubtractRegSwapped(x, y)),
                0xE => Ok(Instruction::ShiftLeft(x, y)),
                _ => Err(()),
            },
            0x9 => Ok(Instruction::SkipIfNotEqualReg(x, y)),
            0xA => Ok(Instruction::LoadAddress(nnn)),
            0xB => Ok(Instruction::JumpV0(nnn)),
            0xC => Ok(Instruction::Random(x, kk)),
            0xD => Ok(Instruction::Draw(x, y, n)),
            0xE => match raw[1] {
                0x9E => Ok(Instruction::SkipIfPressed(x)),
                0xA1 => Ok(Instruction::SkipIfNotPressed(x)),
                _ => Err(()),
            },
            0xF => match raw[1] {
                0x07 => Ok(Instruction::LoadRegDelay(x)),
                0x0A => Ok(Instruction::LoadKey(x)),
                0x15 => Ok(Instruction::LoadDelayReg(x)),
                0x18 => Ok(Instruction::LoadSoundReg(x)),
                0x1E => Ok(Instruction::AddAddress(x)),
                0x29 => Ok(Instruction::LoadAddressDigit(x)),
                0x33 => Ok(Instruction::LoadMemoryBcd(x)),
                0x55 => Ok(Instruction::LoadMemoryRegisters(x)),
                0x65 => Ok(Instruction::LoadRegistersMemory(x)),
                _ => Err(()),
            },
            _ => Err(()),
        };
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::ClearScreen() => write!(f, "CLS"),
            Instruction::SubroutineReturn() => write!(f, "RET"),
            Instruction::Jump(addr) => write!(f, "JP {:#06X}", addr),
            Instruction::SubroutineCall(addr) => write!(f, "CALL {:#06X}", addr),
            Instruction::SkipIfEqualImm(vx, kk) => write!(f, "SE V{:02}, {:#04X}", vx, kk),
            Instruction::SkipIfNotEqualImm(vx, kk) => write!(f, "SNE V{:02}, {:#04X}", vx, kk),
            Instruction::SkipIfEqualReg(vx, vy) => write!(f, "SE V{:02}, V{:02}", vx, vy),
            Instruction::LoadRegImm(vx, kk) => write!(f, "LD V{:02}, {:#04X}", vx, kk),
            Instruction::AddImm(vx, kk) => write!(f, "ADD V{:02}, {:#04X}", vx, kk),
            Instruction::LoadRegReg(vx, vy) => write!(f, "LD V{:02}, V{:02}", vx, vy),
            Instruction::OrReg(vx, vy) => write!(f, "OR V{:02}, V{:02}", vx, vy),
            Instruction::AndReg(vx, vy) => write!(f, "AND V{:02}, V{:02}", vx, vy),
            Instruction::XorReg(vx, vy) => write!(f, "XOR V{:02}, V{:02}", vx, vy),
            Instruction::AddReg(vx, vy) => write!(f, "ADD V{:02}, V{:02}", vx, vy),
            Instruction::SubtractReg(vx, vy) => write!(f, "SUB V{:02}, V{:02}", vx, vy),
            Instruction::ShiftRight(vx, vy) => write!(f, "SHR V{:02}, V{:02}", vx, vy),
            Instruction::SubtractRegSwapped(vx, vy) => write!(f, "SUBN V{:02}, V{:02}", vx, vy),
            Instruction::ShiftLeft(vx, vy) => write!(f, "SHL V{:02}, V{:02}", vx, vy),
            Instruction::SkipIfNotEqualReg(vx, vy) => write!(f, "SNE V{:02}, V{:02}", vx, vy),
            Instruction::LoadAddress(addr) => write!(f, "LD I, {:#06X}", addr),
            Instruction::JumpV0(addr) => write!(f, "JP V0, {:#06X}", addr),
            Instruction::Random(vx, kk) => write!(f, "RND V{:02}, {:#04X}", vx, kk),
            Instruction::Draw(vx, vy, n) => write!(f, "DRW V{:02}, V{:02}, {:#X}", vx, vy, n),
            Instruction::SkipIfPressed(vx) => write!(f, "SKP V{:02}", vx),
            Instruction::SkipIfNotPressed(vx) => write!(f, "SKNP V{:02}", vx),
            Instruction::LoadRegDelay(vx) => write!(f, "LD V{:02}, DT", vx),
            Instruction::LoadKey(vx) => write!(f, "LD V{:02}, K", vx),
            Instruction::LoadDelayReg(vx) => write!(f, "LD DT, V{:02}", vx),
            Instruction::LoadSoundReg(vx) => write!(f, "LD ST, V{:02}", vx),
            Instruction::AddAddress(vx) => write!(f, "ADD I, V{:02}", vx),
            Instruction::LoadAddressDigit(vx) => write!(f, "LD F, V{:02}", vx),
            Instruction::LoadMemoryBcd(vx) => write!(f, "LD B, V{:02}", vx),
            Instruction::LoadMemoryRegisters(vx) => write!(f, "LD [I], V{:02}", vx),
            Instruction::LoadRegistersMemory(vx) => write!(f, "LD V{:02}, [I]", vx),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_nibble() {
        assert_eq!(first_nibble(0xAB), 0xA);
        assert_eq!(first_nibble(0x05), 0x0);
        assert_eq!(first_nibble(0xF5), 0xF);
    }

    #[test]
    fn test_last_nibble() {
        assert_eq!(last_nibble(0xAB), 0xB);
        assert_eq!(last_nibble(0x50), 0x0);
        assert_eq!(last_nibble(0x5F), 0xF);
    }

    #[test]
    fn test_address() {
        assert_eq!(address([0xAB, 0x56]), 0xB56);
        assert_eq!(address([0x50, 0x00]), 0x000);
        assert_eq!(address([0x5F, 0xFF]), 0xFFF);
    }
}
