use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::{self, Display};
use std::io::{self, Write};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    process::exit,
};

enum Register {
    AL,
    AX,
    CL,
    CX,
    DL,
    DX,
    BL,
    BX,
    AH,
    SP,
    CH,
    BP,
    DH,
    SI,
    BH,
    DI,
}

impl Register {
    pub fn create(reg: u8, w: u8) -> Register {
        match w {
            0 => match reg {
                0 => Register::AL,
                1 => Register::CL,
                2 => Register::DL,
                3 => Register::BL,
                4 => Register::AH,
                5 => Register::CH,
                6 => Register::DH,
                7 => Register::BH,
                _ => exit(1),
            },
            1 => match reg {
                0 => Register::AX,
                1 => Register::CX,
                2 => Register::DX,
                3 => Register::BX,
                4 => Register::SP,
                5 => Register::BP,
                6 => Register::SI,
                7 => Register::DI,
                _ => exit(1),
            },
            _ => exit(1),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Register::AL => "al",
            Register::AX => "ax",
            Register::CL => "cl",
            Register::CX => "cx",
            Register::DL => "dl",
            Register::DX => "dx",
            Register::BL => "bl",
            Register::BX => "bx",
            Register::AH => "ah",
            Register::SP => "sp",
            Register::CH => "ch",
            Register::BP => "bp",
            Register::DH => "dh",
            Register::SI => "si",
            Register::BH => "bh",
            Register::DI => "di",
        };
        write!(f, "{}", s)
    }
}

enum Opcode {
    MOV,
}

impl Opcode {
    pub fn new(opcode: u8) -> Opcode {
        match opcode {
            34 => Opcode::MOV,
            _ => exit(1),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Opcode::MOV => "mov",
        };
        write!(f, "{}", s)
    }
}

fn main() -> Result<()> {
    let input_file = "listing_0038_many_register_mov";
    let f = File::open(input_file)?;
    let mut reader = BufReader::new(f);
    let output_file = File::create(format!("{}.asm", input_file))?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "bits 16")?;
    writeln!(writer, "")?;
    loop {
        let binary = match reader.read_u16::<BigEndian>() {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        };

        let byte_1 = ((binary & 0b11111111_00000000) >> 8) as u8;
        let byte_2 = binary as u8;

        let opcode_bytes = (byte_1 & 0b11111100) >> 2; // TODO: cannot always shift by 2
        let opcode = Opcode::new(opcode_bytes);
        let d = byte_1 & (0b00000010);
        let w = byte_1 & (0b00000001);
        let r#mod = (byte_2 & 0b11000000) >> 6;
        let reg = (byte_2 & (0b00111000)) >> 3;
        let rm = byte_2 & (0b00000111);

        if r#mod == 0b11 {
            // REGISTER MODE
            let source_reg: Register;
            let dest_reg: Register;
            if d == 0 {
                source_reg = Register::create(reg, w);
                dest_reg = Register::create(rm, w);
            } else {
                source_reg = Register::create(rm, w);
                dest_reg = Register::create(reg, w);
            }

            writeln!(writer, "mov {}, {}", dest_reg, source_reg)?;
        }
    }
    Ok(())
}
