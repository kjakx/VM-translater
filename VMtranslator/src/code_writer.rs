use std::io::{BufWriter, Write};
use std::fs::File;

pub struct CodeWriter {
    filename: String,
    writer: BufWriter<File>,
    line_count: usize,
}

impl CodeWriter {
    pub fn new(f: File) -> Self {
        CodeWriter {
            filename: String::new(),
            writer: BufWriter::<File>::new(f),
            line_count: 0,
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
        writeln!(self.writer, "@256").unwrap();
        writeln!(self.writer, "D=A").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 4;
    }

    pub fn write_arithmetic(&mut self, command: String) {
        // translate arithmetic command to .asm
        match command.as_str() {
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
                writeln!(self.writer, "D=M").unwrap(); // d = y
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M-D").unwrap(); // d = x - y
                self.line_count += 6;
                writeln!(self.writer, "@{}", self.line_count+7).unwrap();
                writeln!(self.writer, "D;JEQ").unwrap(); // x = y ?
                // false
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "A=M").unwrap();
                writeln!(self.writer, "M=0").unwrap();
                self.line_count += 5;
                writeln!(self.writer, "@{}", self.line_count+5).unwrap();
                writeln!(self.writer, "0;JMP").unwrap();
                // true
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "A=M").unwrap();
                writeln!(self.writer, "M=-1").unwrap();
                // end
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 7;
            },
            "gt" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M-D").unwrap();
                self.line_count += 6;
                writeln!(self.writer, "@{}", self.line_count+7).unwrap();
                writeln!(self.writer, "D;JGT").unwrap(); // x > y ?
                // false
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "A=M").unwrap();
                writeln!(self.writer, "M=0").unwrap();
                self.line_count += 5;
                writeln!(self.writer, "@{}", self.line_count+5).unwrap();
                writeln!(self.writer, "0;JMP").unwrap();
                // true
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "A=M").unwrap();
                writeln!(self.writer, "M=-1").unwrap();
                // end
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 7;
            },
            "lt" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M-D").unwrap();
                self.line_count += 6;
                writeln!(self.writer, "@{}", self.line_count+7).unwrap();
                writeln!(self.writer, "D;JLT").unwrap(); // x < y ?
                // false
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "A=M").unwrap();
                writeln!(self.writer, "M=0").unwrap();
                self.line_count += 5;
                writeln!(self.writer, "@{}", self.line_count+5).unwrap();
                writeln!(self.writer, "0;JMP").unwrap();
                // true
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "A=M").unwrap();
                writeln!(self.writer, "M=-1").unwrap();
                // end
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
                self.line_count += 7;
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
            _ => {
                panic!("invalid arithmetic command");
            },
        }
    }

    pub fn write_push_pop(&mut self, command: String, segment: String, index: i16) {
        // Push or Pop command to .asm
        match command.as_str() {
            "push" => {
                match segment.as_str() {
                    "constant" => {
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=A").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 7;
                    },
                    _ => {
                        unimplemented!();
                    },
                }
            },
            _ => {
                unimplemented!();
            },
        }
    }

    pub fn close(&mut self) {
        writeln!(self.writer, "@{}", self.line_count).unwrap();
        writeln!(self.writer, "0;JMP").unwrap();
        self.writer.flush().unwrap();
    }
}