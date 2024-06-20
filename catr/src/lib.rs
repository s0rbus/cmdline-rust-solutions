use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    ///Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    ///print line numbers
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    pub number_lines: bool,

    ///print nonblank lines
    #[arg(short('b'), long("number-nonblank"))]
    pub number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Args) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(f) => {
                let lines_iter = f.lines().map(|l| l.unwrap());
                let mut linenum = 1;
                for line in lines_iter {
                    if config.number_lines || config.number_nonblank_lines {
                        if config.number_nonblank_lines && line.is_empty() {
                            println!();
                            continue;
                        }
                        print!("     {}\t", linenum);
                        linenum += 1;
                    }
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}
