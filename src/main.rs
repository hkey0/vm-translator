mod code_writer;
mod parser;

use clap::Parser as ClapParser;
use parser::Parser;
use std::fs::File;
use std::io::{Result, Write};

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file.
    #[arg(short, long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let mut parser = Parser::new(&args.file_name);
    let project_name = &args.file_name.replace(".vm", "");
    let mut cw = code_writer::CodeWriter::new(project_name.to_string());
    let mut lines = vec![];

    while parser.has_more_commands() {
        lines.push(format!("// {}", parser.lines[0]));
        parser.advance();
        let cmd = cw.advance(parser.command.clone(), parser.arg1.clone(), parser.arg2);
        println!("{:?}", cmd);
        lines.extend(cmd);
    }

    let mut file = File::create(format!("{}.asm", project_name)).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap()
    }
}
