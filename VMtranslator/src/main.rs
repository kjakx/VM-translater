mod parser;
mod code_writer;

use std::env;
use std::fs::File;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: VMtranslator <filename>.vm | <dirname>"); }
    
    let arg_path = Path::new(&args[1]);

    let mut fin_paths = vec![];
    let fout_path = if arg_path.is_dir() {
        for entry in arg_path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if entry.path().extension().unwrap() == "vm" {
                    fin_paths.push(entry.path());
                }
            }
        }
        let dir_name = arg_path.file_name().unwrap();
        arg_path.join(dir_name).with_extension("asm")
    } else if arg_path.is_file() {
        fin_paths.push((*arg_path).to_path_buf());
        arg_path.with_extension("asm")
    } else {
        panic!("file or directory not found.");
    };

    let fout = File::create(fout_path).unwrap();
    let mut w = code_writer::CodeWriter::new(fout);
    w.write_init();

    for fin_path in fin_paths.iter() {
        let fin = File::open(fin_path)?;
        let mut p = parser::Parser::new(fin);
        w.set_filename(fin_path.file_stem().unwrap().to_string_lossy().to_string());

        while p.has_more_commands() {
            p.advance();
            match p.command_type() {
                parser::CommandType::Arithmetic => {
                    w.write_arithmetic(p.arg1());
                },
                parser::CommandType::Push => {
                    w.write_push_pop(String::from("push"), p.arg1(), p.arg2());
                },
                parser::CommandType::Pop => {
                    w.write_push_pop(String::from("pop"), p.arg1(), p.arg2());
                },
                parser::CommandType::Label => {
                    w.write_label(p.arg1());
                },
                parser::CommandType::Goto => {
                    w.write_goto(p.arg1());
                },
                parser::CommandType::If => {
                    w.write_if(p.arg1());
                },
                parser::CommandType::Function => {
                    w.write_function(p.arg1(), p.arg2());
                },
                parser::CommandType::Return => {
                    w.write_return();
                },
                parser::CommandType::Call => {
                    w.write_call(p.arg1(), p.arg2());
                },
            }
        }
    }

    w.close();
    Ok(())
}
