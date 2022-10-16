use std::{
    error::Error,
    io::{BufRead, Write},
};

use clap::Parser;

/// The rev utility copies the specified files to standard output, reversing the order of
/// characters in every line. If no files are specified, standard input is read.
#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(name = "file")]
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.files.as_slice() {
        [] => {
            for line in std::io::stdin().lines() {
                let mut line = line?.as_bytes().to_owned();
                line.reverse();
                std::io::stdout().lock().write_all(&line)?;
                std::io::stdout().lock().write_all(b"\n")?;
                std::io::stdout().lock().flush()?;
            }
        }
        args => {
            for file in args {
                let file = std::fs::File::open(file)?;
                for line in std::io::BufReader::new(file).lines() {
                    let mut line = line?.as_bytes().to_owned();
                    line.reverse();
                    std::io::stdout().lock().write_all(&line)?;
                    std::io::stdout().lock().write_all(b"\n")?;
                    std::io::stdout().lock().flush()?;
                }
            }
        }
    }

    Ok(())
}
