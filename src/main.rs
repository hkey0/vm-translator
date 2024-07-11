use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file.
    #[arg(short, long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
