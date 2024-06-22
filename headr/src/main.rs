use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::Result;
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
        long,
        value_name = "LINES",
        conflicts_with("bytes"),
        default_value_t = 10,
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64,

    ///Number of bytes to print
    #[arg(short('c'), value_name = "BYTES", long, value_parser = clap::value_parser!(u64).range(1..),)]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    //println!("{:#?}", args);
}

fn run(args: Args) -> Result<()> {
    let numfiles = args.files.len();
    let numbytes = args.bytes.unwrap_or(0);
    for (pos, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                //'head'-style header if multiple files
                if numfiles > 1 {
                    if pos > 0 {
                        println!();
                    }
                    println!("==> {filename} <==");
                }
                if numbytes > 0 {
                    //let mut buffer = vec![0; numbytes as usize];
                    //let bytes_read = file.read(&mut buffer)?;
                    let bytes_read = file
                        .bytes()
                        .take(numbytes as usize)
                        .collect::<Result<Vec<_>, _>>();
                    match bytes_read {
                        Err(err) => eprintln!("reading bytes: {}", err),
                        Ok(b) => print!("{}", String::from_utf8_lossy(&b[..])),
                    }
                    //print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    //this version will include CR which is deliberately in the three.txt file
                    //see book pp 84,85
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                    /* for (line_num, line) in file.lines().enumerate() {
                        match line {
                            Err(err) => {
                                eprintln!("{filename}: {err}");
                                break;
                            }
                            Ok(l) => println!("{l}"),
                        }

                        if line_num + 1 >= args.lines.try_into().unwrap() {
                            break;
                        }
                    } */
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
