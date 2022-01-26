use std::ops::Range;


pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn load(&mut self, memory: &[u8], position: u16) {
        for (index, byte) in memory.iter().enumerate() {
            self.memory[position as usize + index] = *byte
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        self.write(address, (data & 0xFF) as u8);
        self.write(address + 1, (data >> 8) as u8)

    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address + 1) as u16;
        (high << 8) | low
    }

    pub fn slice(&self, range: Range<usize>) -> &[u8] {
        &self.memory[range]
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}