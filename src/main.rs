use anyhow::Result;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
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

fn get_effective_address(rm: u8, r#mod: u8) -> String {
    todo!()
}

fn main() -> Result<()> {
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

        match byte_1 {
            0b10001000..=0b10001011 => {
                match r#mod {
                    0b11 => {
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
                    0b00 => {
                        // Memmory mode, no displacement*
                        match rm {
                            0b000 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + si]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + si]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b001 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + di]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + di]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b010 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + si]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + si]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b011 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + di]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + di]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b100 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[si]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[si]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b101 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[di]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[di]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b110 => {
                                unimplemented!("direct address")
                            }
                            0b111 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx]");
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            _ => unreachable!("R/M field is only 3 bits"),
                        }
                    }
                    0b01 => {
                        let low_byte = reader.read_u8()?;
                        match rm {
                            0b000 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + si + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + si + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b001 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + di + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + di + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b010 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + si + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + si + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b011 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + di + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + di + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b100 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[si + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[si + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b101 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[di + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[di + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b110 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b111 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + {}]", low_byte);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            _ => unreachable!("R/M field is only 3 bits"),
                        }
                    }
                    0b10 => {
                        let disp = reader.read_u16::<LittleEndian>()?;

                        match rm {
                            0b000 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + si + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + si + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b001 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + di + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + di + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b010 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + si + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + si + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b011 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + di + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + di + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b100 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[si + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[si + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b101 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[di + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[di + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b110 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bp + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bp + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            0b111 => {
                                if d == 0 {
                                    let source = Register::create(reg, w);
                                    let dest = format!("[bx + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                } else {
                                    let dest = Register::create(reg, w);
                                    let source = format!("[bx + {}]", disp);
                                    writeln!(writer, "mov {}, {}", dest, source)?;
                                }
                            }
                            _ => unreachable!("R/M field is only 3 bits"),
                        }
                    }
                    _ => unreachable!("Mod field is only 2 bits"),
                }
            }
            0b10110000..=0b10111111 => {}
            _ => unimplemented!("unimplemented instruction decodings"),
        }
    }
    Ok(())
}
