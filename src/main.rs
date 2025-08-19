mod args;
mod pca;
mod read_csv;

use args::Args;
use read_csv::read_csv;
use std::env;
use std::process;

fn main() {
    let args = env::args();

    let config = Args::build(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let data = read_csv(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Error reading CSV file: {}", err);
        process::exit(1);
    });
}
