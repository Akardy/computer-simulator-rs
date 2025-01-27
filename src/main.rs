mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;

fn main() {
    let mut memory = Memory::new();
    let mut cpu = CPU::new(memory);
}
