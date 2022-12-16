use clap::{Arg, Command};
use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};
use std::io::Read;

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
    let lines: usize = match matches.get_one::<String>("number") {
        Some(number) => number.parse::<usize>().unwrap(),
        None => 10,
    };

    //let bytes = match matches.get_one::<>()

    Ok(Config {
        files,
        lines,
        bytes: match matches.get_one::<String>("bytes") {
            Some(bytes) => Some(bytes.parse::<usize>().unwrap()),
            None => None
        },
    })
}

pub fn run (config: Config) -> MyResult<()> {
    let file_count = config.files.len();
    let mut current_file = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprint!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if file_count > 0 as usize {
                    if current_file > 0 {
                        println!();
                    }
                    println!("==> {} <==", filename);
                }
                match config.bytes {
                    Some(bytes) => {
                        let mut buffer = vec![0u8; bytes];
                        file.read(&mut buffer).unwrap();
                        let content = String::from_utf8(buffer)?;
                        print!("{}", content);
                    },
                    None => {
                        for line in file.lines().take(config.lines) {
                            println!("{}", line?);
                        }
                    }
                }
            }
        }
        current_file += 1;
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}