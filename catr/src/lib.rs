use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
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
                .takes_value(false),
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
    /*  if let Some(f) = matches.values_of_lossy("files") {
        let c = Config {
            files: f,
            number_lines: matches.is_present("number_lines"),
            number_nonblank_lines: matches.is_present("number_non-blank_lines"),
        };
        Ok(c)
    } else {
        let e: Box<dyn Error> = String::from("no files given").into();
        Err(e)
    } */
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
