use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

pub struct CodeWriter {
    filename: Path,
    writer: BufWriter<File>,
    line_count: usize,
}

impl CodeWriter {
    pub fn new(p: Path) -> Self {
        let filename = p;
        let fout_path = fin_path.with_extension("asm");
        let fout = File::create(fout_path).unwrap();
        let mut writer = BufWriter::<File>::new(f);
        CodeWriter {
            filename: filename,
            writer: writer,
            usize: 0,
        }
    }

    pub fn set_filename(&self, filename: Path) {
        self.filename = filename;
    }

    pub fn write_arithmetic(&self, command: String) {
        // translate arithmetic command to .asm
        match command {
            "add" => {
                writeln!(self.writer, "@SP").unwrap();    // a = 0
                writeln!(self.writer, "AM=M-1").unwrap(); // m[0] = m[0] - 1, a = m[0] - 1(means a = SP - 1)
                writeln!(self.writer, "D=M").unwrap();    // d = m[SP - 1]
                writeln!(self.writer, "@SP").unwrap();    // a = 0
                writeln!(self.writer, "AM=M-1").unwrap(); // m[0] = m[0] - 1, a = m[0] - 1
                writeln!(self.writer, "M=D+M").unwrap();  // m[SP - 2] = d + m[SP - 2]
                writeln!(self.writer, "@SP").unwrap();    // a = 0
                writeln!(self.writer, "M=M+1").unwrap();  // m[0] = m[0] + 1
                self.line_count += 8;
            },
            "sub" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "M=M-D").unwrap(); // x - y
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 8;
            },
            "neg" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=!M").unwrap(); // two's complement
                writeln!(self.writer, "M=D+1").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 6;
            },
            "eq" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M-D").unwrap();
                self.line_count += 6;
                writeln!(self.writer, "@{}", self.line_count+5).unwrap();
                writeln!(self.writer, "D;JEQ").unwrap(); // x = y ?
                // false
                writeln!(self.writer, "M=0").unwrap();
                self.line_count += 3;
                writeln!(self.writer, "@{}", self.line_count+3).unwrap();
                writeln!(self.writer, "0;JMP").unwrap();
                // true
                writeln!(self.writer, "M=-1").unwrap();
                // end
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 5;
            },
            "gt" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M-D").unwrap();
                self.line_count += 6;
                writeln!(self.writer, "@{}", self.line_count+5).unwrap();
                writeln!(self.writer, "D;JGT").unwrap(); // x > y ?
                // false
                writeln!(self.writer, "M=0").unwrap();
                self.line_count += 3;
                writeln!(self.writer, "@{}", self.line_count+3).unwrap();
                writeln!(self.writer, "0;JMP").unwrap();
                // true
                writeln!(self.writer, "M=-1").unwrap();
                // end
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 5;
            },
            "lt" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M-D").unwrap();
                self.line_count += 6;
                writeln!(self.writer, "@{}", self.line_count+5).unwrap();
                writeln!(self.writer, "D;JLT").unwrap(); // x < y ?
                // false
                writeln!(self.writer, "M=0").unwrap();
                self.line_count += 3;
                writeln!(self.writer, "@{}", self.line_count+3).unwrap();
                writeln!(self.writer, "0;JMP").unwrap();
                // true
                writeln!(self.writer, "M=-1").unwrap();
                // end
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 5;
            },
            "and" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "M=M&D").unwrap(); // x & y
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 8;
            },
            "or" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "M=M|D").unwrap(); // x | y
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 8;
            },
            "not" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "M=!M").unwrap(); // !x
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 5;
            },
            _ => panic!("invalid arithmetic command");
        }
    }

    pub fn write_push_pop(&self, command: String, segment: String, index: i16) {
        // Push or Pop command to .asm
        match command {
            "push" => {
                match segment => {
                    "constant" => {
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=A").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                    },
                    _ => unimplemented!();
                }
            },
            _ => unimplemented!();
        }
    }

    pub fn close() {
        self.writer.flush().unwrap();
        // nothing to do because writer will automatically be dropped...
    }
}