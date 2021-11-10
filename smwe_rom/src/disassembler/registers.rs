// This module will probably not define every register of the 65816,
// only those that are needed for disassembly.

/// Accumulator
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ARegister(pub u16);

/// X Index
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct XRegister(pub u16);

/// Y Index
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct YRegister(pub u16);

/// Data Bank
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DBRegister(pub u8);

/// Direct Page
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DPRegister(pub u16);

/// Program Bank
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PBRegister(pub u8);

/// Program Counter
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PCRegister(pub u8);

/// Processor Status
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PRegister(pub u8);

impl PRegister {
    /// Negative
    pub fn n_flag(&self) -> bool {
        (self.0 & 0b10000000) != 0
    }

    /// Overflow
    pub fn v_flag(&self) -> bool {
        (self.0 & 0b01000000) != 0
    }

    /// Accumulator register size (0 = 16 bit, 1 = 8 bit)
    pub fn m_flag(&self) -> bool {
        (self.0 & 0b00100000) != 0
    }

    /// Index register size (0 = 16 bit, 1 = 8 bit)
    pub fn x_flag(&self) -> bool {
        (self.0 & 0b00010000) != 0
    }

    /// Decimal
    pub fn d_flag(&self) -> bool {
        (self.0 & 0b00001000) != 0
    }

    /// IRQ disable
    pub fn i_flag(&self) -> bool {
        (self.0 & 0b00000100) != 0
    }

    /// Zero
    pub fn z_flag(&self) -> bool {
        (self.0 & 0b00000010) != 0
    }

    /// Carry
    pub fn c_flag(&self) -> bool {
        (self.0 & 0b00000001) != 0
    }
}
