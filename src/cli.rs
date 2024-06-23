use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    lines: Vec<String>,
}

pub fn start() {
    let args = Args::parse();
    println!("{}", args.lines.join(" "))
}