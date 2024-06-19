use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Rowan Limb")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("files to display")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("number lines")
                .takes_value(false)
                .conflicts_with("number_non-blank_lines"),
        )
        .arg(
            Arg::with_name("number_non-blank_lines")
                .short("b")
                .help("number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_non-blank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    //had to explicitly read the struct fields otherwide got a compiler warning about fields never read even with dbg macro)
    //in turn, had to clone for dbg macro which meant deriving Clone
    //println!("Files: {}", config.files.join(","));
    //println!("NumLines: {}", config.number_lines);
    //println!("NumBlankLines: {}", config.number_nonblank_lines);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(f) => {
                println!("Opened {}", filename);
                let lines_iter = f.lines().map(|l| l.unwrap());
                let mut linenum = 1;
                for line in lines_iter {
                    if config.number_lines || config.number_nonblank_lines {
                        if config.number_nonblank_lines && line.is_empty() {
                            println!();
                            continue;
                        }
                        print!("{}\t", linenum);
                        linenum += 1;
                    }
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}
