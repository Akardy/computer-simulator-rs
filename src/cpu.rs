// instructions
// first 3 bits for instructions
/// 0000 - load to a R1: 0000 + the memory address 0000 0000 0000
/// 0001 - store from R1 to memory: 0001 + the memory address 0000 0000 0000
/// 0010 - add: 0010 + the register 00 + xx + a number 0000 0000
/// 0011 - sub: 0011 + the register 00 + xx + a number 0000 0000
/// 0100 - and: 0100 + the register 00 + xx + a number 0000 0000
/// 0101 - move: 0101 + the register 00 + xx + a number 0000 0000
/// 0111 - halt: 0111
/// 
/// registers are represented with 2 bits
/// 00 - R1
/// 01 - R2
/// 10 - R3


use crate::memory::Memory;

pub struct CPU {
    memory: Memory,
    register: [u16; 4],
    pc: u16
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        CPU {
            memory,
            register: [0; 4],
            pc: 0
        }
    }

    pub fn read(&self, index: usize) -> u16 {
        self.register[index]
    }

    pub fn read_memory(&self, address: u16) -> u16{
        self.memory.read(address)
    }

    pub fn write_memory(&mut self, address: u16, data: u16) {
        self.memory.write(address, data);
    }

    /// Programs take the first 512 memory locations
    pub fn execute(&mut self) {
        while self.pc < 512 {
            let instruction = self.memory.read(self.pc);
            let is_halt = self.parse_instruction(&instruction);
            if is_halt {
                break;
            }

            self.pc += 1;
        }
    }

    pub fn parse_instruction(&mut self, instruction: &u16) -> bool {
        let opcode = instruction & 0b1111_0000_0000_0000;
        match opcode {
            0b0000 => {
                let memory_address = instruction & 0b0000_1111_1111_1111;
                let data = self.memory.read(memory_address);
                self.register[0] = data;
            },
            0b0001_0000_0000_0000 => {
                let memory_address = instruction & 0b0000_1111_1111_1111;
                let data = self.register[0];
                self.memory.write(memory_address, data);
            },
            0b0010_0000_0000_0000 => {
                let constant = instruction & 0b0000_0000_1111_1111;
                let register = (instruction & 0b0000_1111_0000_0000) >> 8;
                self.register[register as usize] = self.register[register as usize] + constant;
            },
            0b0011_0000_0000_0000 => {
                let constant = instruction & 0b0000_0000_1111_1111;
                let register = (instruction & 0b0000_1111_0000_0000) >> 8;
                self.register[register as usize] = self.register[register as usize] - constant;
            },
            0b00100_0000_0000_0000 => {
                let constant = instruction & 0b0000_0000_1111_1111;
                let register = (instruction & 0b0000_1111_0000_0000) >> 8;
                self.register[register as usize] = self.register[register as usize] & constant;
            },
            0b00101_0000_0000_0000 => {
                let constant = instruction & 0b0000_0000_1111_1111;
                let register = (instruction & 0b0000_1111_0000_0000) >> 8;
                self.register[register as usize] = constant;
            },
            0b00111_0000_0000_0000 => {
                return true;
            }
            _ => {
                println!("Unknown opcode!")
            }
        }
        false
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn write_ram() {
        let mut ram = Memory::new();

        let data = 0b0000111101011111;
        let address = 0b01;
        ram.write(address, data);

        assert_eq!(ram.memory[address as usize], 0b0000111101011111);
    }

    #[test]
    fn load() {
        let mut ram = Memory::new();
        let mut cpu = CPU::new(ram);

        let data = 0b0000_1111_1101_0111;
        let address = 0b0000_0000_0000_0011;
        cpu.memory.write(address, data);

        let instruction: u16 = 0b0000_0000_0000_0011;
        cpu.parse_instruction(&instruction);

        assert_eq!(cpu.register[0], data);
        assert_eq!(cpu.register[0], cpu.memory.read(address));

    }
}