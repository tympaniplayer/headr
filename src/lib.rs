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
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .help("print the first n lines of a file instead of 10")
                .conflicts_with("bytes")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .help("Print bytes of each of the specified files"),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("file")
        .unwrap()
        .map(|v| v.to_string())
        .collect();

    let lines = matches
        .get_one::<String>("lines")
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;


    let bytes = matches
        .get_one::<String>("bytes")
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files,
        lines: lines.unwrap(),
        bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val))
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprint!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    )
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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