use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use log::info;
use std::env;
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
    pub fn new(reg: u8, w: u8) -> Register {
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

#[allow(unused)]
enum Opcode {
    MOV,
}

struct Memory {}

impl Memory {
    pub fn get_effective_address(rm: u8, r#mod: u8) -> String {
        match rm {
            0b000 => String::from("bx + si"),
            0b001 => String::from("bx + di"),
            0b010 => String::from("bp + si"),
            0b011 => String::from("bp + di"),
            0b100 => String::from("si"),
            0b101 => String::from("di"),
            0b110 => {
                if r#mod == 0b00 {
                    unimplemented!("direct address");
                } else {
                    String::from("bp")
                }
            }
            0b111 => String::from("bx"),
            _ => unimplemented!("rm must be between 0 and 7"),
        }
    }
    pub fn new(rm: u8, r#mod: u8, low_data: Option<u8>, high_data: Option<u16>) -> String {
        let default = Memory::get_effective_address(rm, r#mod);
        match r#mod {
            0b00 => {
                format!("[{}]", default)
            }
            0b01 => {
                format!("[{} + {}]", default, low_data.unwrap())
            }
            0b10 => {
                format!("[{} + {}]", default, high_data.unwrap())
            }
            _ => unimplemented!("mod is only 2 bits"),
        }
    }
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

#[allow(dead_code, unused)]
fn get_effective_address(rm: u8, r#mod: u8) -> String {
    todo!()
}

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <argument>", args[0]);
        return Ok(());
    }

    let input_file = args[1].clone();
    let f = File::open(&input_file)?;
    let mut reader = BufReader::new(f);
    let output_file = File::create(format!("{}.asm", input_file))?;
    let mut writer = BufWriter::new(output_file);

    writeln!(writer, "bits 16")?;
    writeln!(writer, "")?;
    // 10001011
    loop {
        let byte_1 = match reader.read_u8() {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e.into()),
        };

        match byte_1 {
            0b10001000..=0b10001011 => {
                // Register/Memory to/from register
                let byte_2 = reader.read_u8()?;
                let d = byte_1 & (0b00000010);
                let w = byte_1 & (0b00000001);

                let r#mod = (byte_2 & 0b11000000) >> 6;
                let reg = (byte_2 & (0b00111000)) >> 3;
                let rm = byte_2 & (0b00000111);

                let source: String;
                let dest: String;
                match r#mod {
                    0b11 => {
                        // REGISTER MODE
                        if d == 0 {
                            source = Register::new(reg, w).to_string();
                            dest = Register::new(rm, w).to_string();
                        } else {
                            source = Register::new(rm, w).to_string();
                            dest = Register::new(reg, w).to_string();
                        }
                    }
                    0b00 => {
                        // Memmory mode, no displacement*
                        if d == 0 {
                            source = Register::new(reg, w).to_string();
                            dest = Memory::new(rm, r#mod, None, None);
                        } else {
                            dest = Register::new(reg, w).to_string();
                            source = Memory::new(rm, r#mod, None, None);
                        }
                    }
                    0b01 => {
                        let low_byte = reader.read_u8()?;
                        if d == 0 {
                            source = Register::new(reg, w).to_string();
                            dest = Memory::new(rm, r#mod, Some(low_byte), None);
                        } else {
                            dest = Register::new(reg, w).to_string();
                            source = Memory::new(rm, r#mod, Some(low_byte), None);
                        }
                    }
                    0b10 => {
                        let disp = reader.read_u16::<LittleEndian>()?;
                        if d == 0 {
                            source = Register::new(reg, w).to_string();
                            dest = Memory::new(rm, r#mod, None, Some(disp));
                        } else {
                            dest = Register::new(reg, w).to_string();
                            source = Memory::new(rm, r#mod, None, Some(disp));
                        }
                    }
                    _ => unimplemented!("mod is only 2 bits"),
                }
                writeln!(writer, "mov {}, {}", dest, source)?;
            }
            0b10110000..=0b10111111 => {
                // Immediate to register
                let w = (byte_1 & 0b00001000) >> 3;
                let reg_bytes = byte_1 & 0b00000111;
                let reg = Register::new(reg_bytes, w);

                match w {
                    0 => {
                        let data = reader.read_u8()?;
                        writeln!(writer, "mov {}, {}", reg, data)?;
                    }
                    1 => {
                        let data = reader.read_u16::<LittleEndian>()?;
                        writeln!(writer, "mov {}, {}", reg, data)?;
                    }
                    _ => unreachable!("Width is only 0 or 1"),
                }
            }
            _ => unimplemented!("unimplemented instruction decodings"),
        }
    }
    Ok(())
}
