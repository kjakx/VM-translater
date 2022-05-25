mod parser;
mod code_writer;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: VMtranslator { <filename>.vm | <dirname> }"); }
    
    let fpath = Path::new(&args[1]);
    // TODO: deal with both of file and directory
    let fin = File::open(fpath)?;
    let mut p = parser::Parser::new(fin);
    let mut w = code_writer::CodeWriter::new(fpath);

    while p.has_more_commands() {
        p.advance();
        match p.command_type() {
            CommandType::Arithmetic => {
                write_arithmetic(p.arg1());
            },
            CommandType::Push => {
                write_push_pop("push", p.arg1(), p.arg2());
            },
            CommandType::Pop => {
                write_push_pop("pop", p.arg1(), p.arg2());
            },
            _ => unimplemented!();
        }
    }
}
