use clap::{Arg, ArgAction, Command};
use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.1")
        .author("Nate Palmer <nate@natepalmer.dev")
        .about("Rust cat")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .help("print the first n lines of a file instead of 10")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .help("Print bytes of each of the specified files"),
        )
        .get_matches();
    let files = matches
        .get_many::<String>("file")
        .unwrap()
        .map(|v| v.to_string())
        .collect();
    let lines: usize = match matches.get_one::<usize>("number").copied() {
        Some(number) => number,
        None => 10,
    };

    Ok(Config {
        files,
        lines,
        bytes: matches.get_one::<usize>("bytes").copied(),
    })
}

pub fn run (config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprint!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                match config.bytes {
                    Some(bytes) => (),
                    None => {
                        for line in file.lines().take(config.lines) {
                            println!("{}", line.unwrap());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}