use std::env;
use std::process;
use std::path::Path;

use vm_translator::{Parser, CodeWriter};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = vm_translator::parse_input(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // TODO: add implementation for directory with multiple files
    let out_filename = filename.replace(".vm", ".asm");
    let out_filepath = Path::new(&out_filename);

    let mut code_writer = CodeWriter::new(out_filepath);

    let content = vm_translator::read_file(filename).unwrap_or_else(|err| {
        println!("Application error: {}", err);

        process::exit(1);
    });

    println!("{}", content);

    let name = filename.split("/").last().unwrap().replace(".vm", "");
    let mut parser = Parser::new(&name, &content, &mut code_writer);
    parser.to_assembler();

    code_writer.close();
}
