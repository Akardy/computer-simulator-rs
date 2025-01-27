use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn get_register(register: &str) -> u16 {
    match register {
        "r1" => 0b0000_0000_0000_0000,
        "r2" => 0b0000_0001_0000_0000,
        "r3" => 0b0000_0010_0000_0000,
        _ => panic!("Register not found")
    }
}

pub fn run(filename: &Path) -> Vec<u16> {
    let mut machine_codes: Vec<u16> = Vec::new();

    let mut file = match File::open(&filename) {
        Err(_e) => panic!("couldn't open"),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_e) => panic!("couldn't read"),
        Ok(_) => {},

    }

    let lines: Vec<&str> = s.lines().collect();

    for line in lines {
        if line.contains(":") {
            continue;
        }

        let instruction: Vec<&str> = line.split(" ").collect();
        
        match instruction[0] {
            "load" => {
                let opcode = 0b0000_0000_0000_0000;
                let address = instruction[2].parse::<u16>().unwrap();
                let full_instruction = opcode | 0b0000_0000_0000_0000 | address; 
                machine_codes.push(full_instruction);
            },
            "store" => {
                let opcode = 0b0001_0000_0000_0000;
                let address = instruction[2].parse::<u16>().unwrap();
                let full_instruction = opcode | 0b0000_0000_0000_0000 | address; 
                machine_codes.push(full_instruction);
            },
            "add" => {
                let opcode = 0b0010_0000_0000_0000;
                let register = get_register(instruction[1]);
                let constant = instruction[2].parse::<u16>().unwrap();
                let full_instruction = opcode | register | constant;
                machine_codes.push(full_instruction);
            },
            "sub" => {
                let opcode = 0b0011_0000_0000_0000;
                let register = get_register(instruction[1]);
                let constant = instruction[2].parse::<u16>().unwrap();
                let full_instruction = opcode | register | constant;
                machine_codes.push(full_instruction);
            },
            "and" => {
                let opcode = 0b0100_0000_0000_0000;
                let register = get_register(instruction[1]);
                let constant = instruction[2].parse::<u16>().unwrap();
                let full_instruction = opcode | register | constant;
                machine_codes.push(full_instruction);
            },
            "move" => {
                let opcode = 0b0101_0000_0000_0000;
                let register = get_register(instruction[1]);
                let constant = instruction[2].parse::<u16>().unwrap();
                let full_instruction = opcode | register | constant;
                machine_codes.push(full_instruction);
            },
            "halt" => {
                let opcode = 0b0111_0000_0000_0000;
                machine_codes.push(opcode);
            }
            _ => {
                continue;
            }
        }

    }

    machine_codes
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn main() {
        let filename = Path::new("programs/test.asm");
        let program = run(filename);
        assert_eq!(program, [20481, 8201, 12292, 4912, 20746, 12549, 12547, 28672]);
    }

}