mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::{Parser, Subcommand};

pub use self::base64::{Base64Format, Base64Subcommand};
pub use self::csv::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about,long_about = None )]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert to CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("file not found"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("no-exist"), Err("file not found"));
    }
}
