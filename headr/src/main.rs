use clap::Parser;

///Rust version of 'head'
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    ///Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    ///Number of lines to print
    #[arg(
        short('n'),
        long("lines"),
        conflicts_with("bytes"),
        default_value_t = 10
    )]
    lines: u64,

    ///Number of bytes to print
    #[arg(short('c'), long("bytes"))]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
