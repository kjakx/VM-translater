use std::io::{BufWriter, Write};
use std::fs::File;

pub struct CodeWriter {
    writer: BufWriter<File>,
    filename: String,
    function_name: String,
    line_count: usize,
    call_count: usize,
}

impl CodeWriter {
    pub fn new(f: File) -> Self {
        CodeWriter {
            writer: BufWriter::<File>::new(f),
            filename: String::new(),
            function_name: String::new(),
            line_count: 0,
            call_count: 0,
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn write_init(&mut self) {
        writeln!(self.writer, "@256").unwrap();
        writeln!(self.writer, "D=A").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 4;
        self.write_call(String::from("Sys.init"), 0);
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
                        writeln!(self.writer, "@{}", 3 + index).unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 7;
                    },
                    "temp" => {
                        writeln!(self.writer, "@{}", 5 + index).unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "A=M").unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "M=M+1").unwrap();
                        self.line_count += 7;
                    },
                    "static" => {
                        writeln!(self.writer, "@{}.{}", self.filename, index).unwrap();
                        writeln!(self.writer, "D=M").unwrap();
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
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", 3 + index).unwrap();
                        writeln!(self.writer, "M=D").unwrap(); // m[THIS + index] = D
                        self.line_count += 5;
                    },
                    "temp" => {
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}", 5 + index).unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 5;
                    },
                    "static" => {
                        writeln!(self.writer, "@SP").unwrap();
                        writeln!(self.writer, "AM=M-1").unwrap();
                        writeln!(self.writer, "D=M").unwrap();
                        writeln!(self.writer, "@{}.{}", self.filename, index).unwrap();
                        writeln!(self.writer, "M=D").unwrap();
                        self.line_count += 5;
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
 
    pub fn write_label(&mut self, label: String) {
        if self.function_name != "" {
            writeln!(self.writer, "({}${})", self.function_name, label).unwrap();
        } else {
            writeln!(self.writer, "({})", label).unwrap();
        }
    }

    pub fn write_goto(&mut self, label: String) {
        if self.function_name != "" {
            writeln!(self.writer, "@{}${}", self.function_name, label).unwrap();
        } else {
            writeln!(self.writer, "@{}", label).unwrap()
        }
        writeln!(self.writer, "0;JMP").unwrap();
        self.line_count += 2;
    }

    pub fn write_if(&mut self, label: String) {
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "AM=M-1").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        if self.function_name != "" {
            writeln!(self.writer, "@{}${}", self.function_name, label).unwrap();
        } else {
            writeln!(self.writer, "@{}", label).unwrap()
        }
        writeln!(self.writer, "D;JNE").unwrap();
        self.line_count += 5;
    }

    pub fn write_call(&mut self, function_name: String, num_args: i16) {
        // push return-address
        writeln!(self.writer, "@Return.{}", self.call_count).unwrap();
        writeln!(self.writer, "D=A").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=M+1").unwrap();
        self.line_count += 7;

        // push LCL
        writeln!(self.writer, "@LCL").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=M+1").unwrap();
        self.line_count += 7;

        // push ARG
        writeln!(self.writer, "@ARG").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=M+1").unwrap();
        self.line_count += 7;

        // push THIS
        writeln!(self.writer, "@THIS").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=M+1").unwrap();
        self.line_count += 7;

        // push THAT
        writeln!(self.writer, "@THAT").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=M+1").unwrap();
        self.line_count += 7;

        // ARG = SP - n - 5
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@{}", num_args).unwrap();
        writeln!(self.writer, "D=D-A").unwrap();
        writeln!(self.writer, "@5").unwrap();
        writeln!(self.writer, "D=D-A").unwrap();
        writeln!(self.writer, "@ARG").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 8;

        // LCL = SP
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@LCL").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 4;

        // goto f
        writeln!(self.writer, "@{}", function_name).unwrap();
        writeln!(self.writer, "0;JMP").unwrap();
        self.line_count += 2;
        writeln!(self.writer, "(Return.{})", self.call_count).unwrap();
        self.call_count += 1;
    }

    pub fn write_return(&mut self) {
        // FRAME = LCL
        writeln!(self.writer, "@LCL").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@R13").unwrap(); // FRAME
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 4;

        // RET = *(FRAME - 5)
        writeln!(self.writer, "@R13").unwrap(); // FRAME
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@5").unwrap(); // FRAME
        writeln!(self.writer, "A=D-A").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@R14").unwrap(); // RET
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 7;

        // *ARG = pop()
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "AM=M-1").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@ARG").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 6;

        // SP = ARG + 1
        writeln!(self.writer, "@ARG").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=D+1").unwrap();
        self.line_count += 4;

        // THAT = *(FRAME - 1)
        writeln!(self.writer, "@R13").unwrap(); // FRAME
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@1").unwrap(); // FRAME
        writeln!(self.writer, "A=D-A").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@THAT").unwrap(); // RET
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 7;

        // THIS = *(FRAME - 2)
        writeln!(self.writer, "@R13").unwrap(); // FRAME
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@2").unwrap(); // FRAME
        writeln!(self.writer, "A=D-A").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@THIS").unwrap(); // RET
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 7;

        // ARG = *(FRAME - 3)
        writeln!(self.writer, "@R13").unwrap(); // FRAME
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@3").unwrap(); // FRAME
        writeln!(self.writer, "A=D-A").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@ARG").unwrap(); // RET
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 7;

        // LCL = *(FRAME - 4)
        writeln!(self.writer, "@R13").unwrap(); // FRAME
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@4").unwrap(); // FRAME
        writeln!(self.writer, "A=D-A").unwrap();
        writeln!(self.writer, "D=M").unwrap();
        writeln!(self.writer, "@LCL").unwrap(); // RET
        writeln!(self.writer, "M=D").unwrap();
        self.line_count += 7;

        // goto RET
        writeln!(self.writer, "@R14").unwrap(); // FRAME
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "0;JMP").unwrap();
        self.line_count += 3;
    }

    pub fn write_function(&mut self, function_name: String, num_locals: i16) {
        self.function_name = function_name;
        // function_name label
        writeln!(self.writer, "({})", self.function_name).unwrap();
        // local variables initialization
        writeln!(self.writer, "@{}", num_locals).unwrap();
        writeln!(self.writer, "D=A").unwrap();
        self.line_count += 2;
        writeln!(self.writer, "@{}", self.line_count+10).unwrap();
        writeln!(self.writer, "D;JEQ").unwrap();
        // push 0 num_locals times
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "A=M").unwrap();
        writeln!(self.writer, "M=0").unwrap();
        writeln!(self.writer, "@SP").unwrap();
        writeln!(self.writer, "M=M+1").unwrap();
        writeln!(self.writer, "D=D-1").unwrap();
        self.line_count += 8;
        writeln!(self.writer, "@{}", self.line_count-8).unwrap();
        writeln!(self.writer, "0;JMP").unwrap();
        self.line_count += 2;
    }

    pub fn close(&mut self) {
        writeln!(self.writer, "@{}", self.line_count).unwrap();
        writeln!(self.writer, "0;JMP").unwrap();
        self.writer.flush().unwrap();
    }
}