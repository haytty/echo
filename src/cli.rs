use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    strings: Vec<String>,
}

pub fn start() {
    let args = Args::parse();
    println!("{}", args.strings.join(" "))
}