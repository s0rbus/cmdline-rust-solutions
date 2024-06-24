use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    ///Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    ///Show line count
    #[arg(short('l'), long)]
    lines: bool,

    ///Show word count
    #[arg(short('w'), long)]
    words: bool,

    ///Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    ///Show char count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    println!("{args:#?}");
    Ok(())
}
