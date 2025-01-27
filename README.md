# Simple Computer Simulator

A basic computer simulator to execute basic instructions using a custom instruction set and memory model.

## Features
- **Instruction Set**:
  - `0000`: Load from memory to R1
  - `0001`: Store from R1 to memory
  - `0010`: Add a constant to a register
  - `0011`: Subtract a constant from a register
  - `0100`: Bitwise AND with a constant
  - `0101`: Move a constant to a register
  - `0111`: Halt execution
- **Registers**: R1, R2, R3 (represented with 2 bits each)
- **Memory**: Programs are stored in the first 512 locations.
