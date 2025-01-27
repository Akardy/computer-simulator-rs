pub const SIZE: usize = 1024;

pub struct Memory {
    pub memory: [u16; SIZE]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; SIZE]
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        let address = address as usize;
        self.memory[address]
    }

    pub fn write(&mut self, address: u16, data: u16) {
        let address = address as usize;
        self.memory[address] = data;
    }
}