use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

pub struct CodeWriter {
    filename: Path,
    writer: BufWriter<File>,
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
        }
    }

    pub fn set_filename(&self, filename: Path) {
        self.filename = filename;
    }

    pub fn write_arithmetic(&self, command: String) {
        // translate arithmetic command to .asm
        match command {
            "add" => {
                writeln!(self.writer, "@SP").unwrap();    // a = 1
                writeln!(self.writer, "AM=M-1").unwrap(); // m[1] = m[1] - 1, a = m[1](means a = SP - 1)
                writeln!(self.writer, "D=M").unwrap();    // d = m[SP - 1]
                writeln!(self.writer, "@SP").unwrap();    // a = 1
                writeln!(self.writer, "AM=M-1").unwrap(); // m[1] = m[1] - 1, a = m[1]
                writeln!(self.writer, "M=D+M").unwrap();  // m[SP - 2] = d + m[SP - 2]
                writeln!(self.writer, "@SP").unwrap();    // a = 1
                writeln!(self.writer, "M=M+1").unwrap();  // m[1] = m[1] + 1
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
            },
            "neg" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=!M").unwrap(); // two's complement
                writeln!(self.writer, "M=D+1").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
            },
            "eq" => {
                unimplemented!();
            },
            "gt" => {
                unimplemented!();
            },
            "lt" => {
                unimplemented!();
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
            },
            "or" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "D=M").unwrap();
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "M=M&D").unwrap(); // x | y
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
            },
            "not" => {
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "AM=M-1").unwrap();
                writeln!(self.writer, "M=!M").unwrap(); // !x
                writeln!(self.writer, "@SP").unwrap();
                writeln!(self.writer, "M=M+1").unwrap();
            },
        }
    }

    pub fn write_push_pop(&self, command: String, segment: String, index: i16) {
        // Push or Pop command to .asm
    }

    pub fn close() {
        self.writer.flush().unwrap();
        // nothing to do because writer will automatically be dropped...
    }
}