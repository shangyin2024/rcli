mod csv;
mod genpass;

use std::path::Path;

use clap::{Parser, Subcommand};

use self::{csv::CsvOpts, genpass::GenPassOpts};

pub use self::csv::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "csv", about = "Show Csv,or covert VSC to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

fn verify_input_file(input_file: &str) -> Result<String, &'static str> {
    // 判断文件是否存在
    if !Path::new(input_file).exists() {
        return Err("Input file does not exist");
    }

    if input_file.is_empty() {
        return Err("Input file is required");
    }
    Ok(input_file.into())
}
