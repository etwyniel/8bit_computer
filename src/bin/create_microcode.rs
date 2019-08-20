use breadboard_8bit::modules::{ControlFlag, ControlWord};
use std::io::{self, BufWriter, Write};

// Argument to decoder is in the form
// F F F F    I I I I   S S S S
// ^ ^ ^ ^    ^ ^ ^ ^   ^ ^ ^ ^
// flags    instruction  step
fn write_microcode<F: Fn(u16) -> u32, W: Write>(writer: &mut W, decoder: F) -> io::Result<()> {
    for i in 0..(1 << 12) {
        let cw = decoder(i);
        writer.write_all(&cw.to_le_bytes())?;
    }
    Ok(())
}

struct MicrocodeAddress {
    flags: u8,
    instruction: u8,
    step: u8,
}

fn unpack_microcode_address(address: u16) -> MicrocodeAddress {
    let flags = ((address >> 8) & 0b1111) as u8;
    let instruction = ((address >> 4) & 0b1111) as u8;
    let step = (address & 0b1111) as u8;
    MicrocodeAddress {
        flags,
        instruction,
        step,
    }
}

fn sample_decoder(address: u16) -> u32 {
    use ControlFlag::*;

    let MicrocodeAddress {
        flags,
        instruction,
        step,
    } = unpack_microcode_address(address);
    let carry = flags & 0b10 > 0;
    let zero = flags & 0b01 > 0;
    match (instruction, step) {
        (_, 0) => CounterOut | MemoryAddressIn,
        (_, 1) => RamOut | InstructionRegisterIn | CounterEnable,

        // LDA
        (0x1, 2) => InstructionRegisterOut | MemoryAddressIn,
        (0x1, 3) => RamOut | ARegisterIn | NextInstruction,

        // ADD
        (0x2, 2) => InstructionRegisterOut | MemoryAddressIn,
        (0x2, 3) => RamOut | BRegisterIn,
        (0x2, 4) => SumOut | ARegisterIn | NextInstruction | FlagRegisterIn,

        // SUB
        (0x3, 2) => InstructionRegisterOut | MemoryAddressIn,
        (0x3, 3) => RamOut | BRegisterIn,
        (0x3, 4) => Subtract | SumOut | ARegisterIn | NextInstruction | FlagRegisterIn,

        // STA
        (0x4, 2) => InstructionRegisterOut | MemoryAddressIn,
        (0x4, 3) => ARegisterOut | RamIn | NextInstruction,

        // LDI
        (0x5, 2) => InstructionRegisterOut | ARegisterIn | NextInstruction,

        //JMP
        (0x6, 2) => InstructionRegisterOut | Jump | NextInstruction,

        // JC
        (0x7, 2) if carry => InstructionRegisterOut | Jump | NextInstruction,
        (0x7, 2) => Empty | NextInstruction,

        // JZ
        (0x8, 2) if zero => InstructionRegisterOut | Jump | NextInstruction,
        (0x8, 2) => Empty | NextInstruction,

        // OUT
        (0xe, 2) => ARegisterOut | OutputRegisterIn | NextInstruction,

        // Hlt
        (0xf, 2) => ControlWord(Hlt as u32),
        _ => ControlWord(0),
    }.0
}

fn main() -> io::Result<()> {
    let file = std::fs::File::create("microcode")?;
    write_microcode(&mut BufWriter::new(file), sample_decoder)
}
