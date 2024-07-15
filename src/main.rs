mod code_writer;
mod parser;

use clap::Parser as ClapParser;
use parser::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file.
    #[arg(short, long)]
    directory: String,
}

fn main() {
    let args = Args::parse();
    let mut parser = Parser::new();
    let project_name = &args.directory;
    let mut cw = code_writer::CodeWriter::new(project_name.to_string());
    let mut lines = vec![];

    boot_stack_pointer(&mut lines);

    let path = Path::new(project_name);
    if !path.is_dir() {
        panic!("Give a valid directory.");
    }
    for entry in fs::read_dir(path).unwrap() {
        let fep = entry.unwrap().path();
        let file_path = fep.to_str().unwrap();

        if file_path.ends_with(".vm") {
            process_file(&mut parser, &mut cw, &mut lines, file_path);
        }
    }

    let mut file = File::create(format!("{}.asm", project_name)).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap()
    }
}

fn process_file(
    parser: &mut Parser,
    cw: &mut code_writer::CodeWriter,
    lines: &mut Vec<String>,
    file_path: &str,
) {
    // set current file for both
    parser.set_file(file_path);
    cw.set_current_file(file_path);
    // add current file as comment line
    lines.push(format!("// {}", file_path));

    // process file
    while parser.has_more_commands() {
        // append current command as comment line
        lines.push(format!("// {}", parser.lines[0]));
        parser.advance();
        let cmd = cw.advance(parser.command.clone(), parser.arg1.clone(), parser.arg2);
        lines.extend(cmd);
    }
}

fn boot_stack_pointer(lines: &mut Vec<String>) {
    lines.push("@256".to_string());
    lines.push("D=A".to_string());
    lines.push("@SP".to_string());
    lines.push("M=D".to_string());
}
