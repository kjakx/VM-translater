use std::io::BufWriter;
use std::io::Write;
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
    }

    pub fn write_push_pop(&self, command: String, segment: String, index: i16) {
        // Push or Pop command to .asm
    }

    pub fn close() {
        self.writer.flush().unwrap();
        // nothing to do because writer will automatically be dropped...
    }
}