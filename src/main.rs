mod code_writer;
mod parser;

use clap::Parser as ClapParser;
use parser::Parser;
use std::fs;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file.
    #[arg(short, long)]
    folder_name: String,
}

fn main() {
    let args = Args::parse();
    let mut parser = Parser::new();
    let project_name = &args.folder_name; // .replace(".vm", "");
    let mut cw = code_writer::CodeWriter::new(project_name.to_string());
    let mut lines = vec![];

    lines.push("@256".to_string());
    lines.push("D=A".to_string());
    lines.push("@SP".to_string());
    lines.push("M=D".to_string());

    let path = Path::new(project_name);
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        println!("{}", file_path.display());
        if file_path.to_str().unwrap().ends_with(".vm") {
            parser.set_file(file_path.to_str().unwrap());
            cw.set_current_file(file_path.to_str().unwrap());
            lines.push(format!("// {}", file_path.to_str().unwrap()).to_string());

            while parser.has_more_commands() {
                lines.push(format!("// {}", parser.lines[0]));
                parser.advance();
                let cmd = cw.advance(parser.command.clone(), parser.arg1.clone(), parser.arg2);
                println!("{:?}", cmd);
                lines.extend(cmd);
            }
        }
    }

    let mut file = File::create(format!("{}.asm", project_name)).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap()
    }
}

fn process_file() {}
