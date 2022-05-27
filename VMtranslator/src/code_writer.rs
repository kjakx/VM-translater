use std::io::{BufWriter, Write};
use std::fs::File;

pub struct CodeWriter {
    filename: String,
    writer: BufWriter<File>,
    line_count: usize,
}

impl CodeWriter {
    pub fn new(f: File) -> Self {
        let mut line_count = 0;
        let mut writer = BufWriter::<File>::new(f);
        writeln!(writer, "@256").unwrap();
        writeln!(writer, "D=A").unwrap();
        writeln!(writer, "@SP").unwrap();
        writeln!(writer, "M=D").unwrap();
        line_count += 4;
        CodeWriter {
            filename: String::new(),
            writer: writer,
            line_count: line_count,
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
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
                    "local" => {
                        writeln!(self.writer, "@LCL").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    "argument" => {
                        writeln!(self.writer, "@ARG").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    "this" => {
                        writeln!(self.writer, "@THIS").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    "that" => {
                        writeln!(self.writer, "@THAT").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    "pointer" => {
                        writeln!(self.writer, "@THIS").unwrap();
                        writeln!(self.writer, "D=A").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    "temp" => {
                        writeln!(self.writer, "@5").unwrap();
                        writeln!(self.writer, "D=A").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    "static" => {
                        writeln!(self.writer, "@{}.{}", self.filename, index).unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "A=D+A").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 10;
                    },
                    _ => {
                        unimplemented!();
                    },
                }
            },
            "pop" => {
                match segment.as_str() {
                    "local" => {
                        writeln!(self.writer, "@LCL").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap(); // a = m[LCL] + index
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "M=D").unwrap(); // m[13] = m[LCL] + index
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap(); // m[LCL + index] = D
                        self.line_count += 12;
                    },
                    "argument" => {
                        writeln!(self.writer, "@ARG").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 12;
                    },
                    "this" => {
                        writeln!(self.writer, "@THIS").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 12;
                    },
                    "that" => {
                        writeln!(self.writer, "@THAT").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 12;
                    },
                    "pointer" => {
                        writeln!(self.writer, "@THIS").unwrap();
                        writeln!(self.writer, "D=A").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap();
                        writeln!(self.writer, "@R13").unwrap(); // m[13] = THIS + index
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap(); // m[THIS + index] = D
                        self.line_count += 12;
                    },
                    "temp" => {
                        writeln!(self.writer, "@5").unwrap();
                        writeln!(self.writer, "D=A").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 12;
                    },
                    "static" => {
                        writeln!(self.writer, "@{}.{}", self.filename, index).unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", index).unwrap();
                        writeln!(self.writer, "D=D+A").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@R13").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 12;
                    },
                    _ => {
                        unimplemented!();
                    },
                }
            },
            _ => {
                panic!("invalid command");
            },
        }
    }

    pub fn close(&mut self) {
        writeln!(self.writer, "@{}", self.line_count).unwrap();
        writeln!(self.writer, "0;JMP").unwrap();
        self.writer.flush().unwrap();
    }
}