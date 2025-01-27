mod cpu;
mod memory;
mod assembler;

use cpu::CPU;
use memory::Memory;
use std::path::Path;

fn main() {
    let filename = Path::new("programs/test.asm");
    let program = assembler::run(filename);
    let memory = Memory::new();
    let mut cpu = CPU::new(memory);

    let mut counter = 0;
    for instruction in program {
        cpu.write_memory(counter, instruction);
        counter += 1;
    }

    cpu.execute();

    println!("r1 value: {}", cpu.read(0));
    println!("r2 value: {}", cpu.read(1));
    println!("M[254] value: {}", cpu.read_memory(254));
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_assembler() {
        let filename = Path::new("programs/test.asm");
        let program = assembler::run(filename);
        let memory = Memory::new();
        let mut cpu = CPU::new(memory);

        for instruction in program {
            let is_halt = cpu.parse_instruction(&instruction);
            if is_halt {
                break;
            }
        }

        assert_eq!(cpu.read(0), 6);
        assert_eq!(cpu.read(1), 2);
        assert_eq!(cpu.read_memory(254), 6);
           
        }

        

}
