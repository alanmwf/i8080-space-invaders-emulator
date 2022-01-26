pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
}

pub enum Flag {
    Sign = 7,
    Zero = 6,
    AuxiliaryCarry = 4,
    Parity = 2,
    Carry = 0,
}

impl Register {
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.flags as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_af(&mut self, data: u16) {
        self.a = (data >> 8) as u8;
        self.flags = (data & 0x00D5 | 0x0002) as u8;
    }

    pub fn set_bc(&mut self, data: u16) {
        self.b = (data >> 8) as u8;
        self.c = (data & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, data: u16) {
        self.d = (data >> 8) as u8;
        self.e = (data & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, data: u16) {
        self.h = (data >> 8) as u8;
        self.l = (data & 0x00FF) as u8;
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags = self.flags | (1 << flag as usize)
        }
        else {
            self.flags = self.flags & !(1 << flag as usize)
        }
    }
    
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.flags & (1 << flag as usize) != 0
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: 0b0000_0010,
            stack_pointer: 0,
            program_counter: 0,
        }
    }
}
