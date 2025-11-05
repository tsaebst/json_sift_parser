use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

// CLI definition
#[derive(Parser, Debug)]
#[command(
    name = "jsonsift",
    author = "Vladyslava Spitkovska",
    version = "1.0",
    about = "JSON-Sift is my first parser. It processes aviation weather data used in civil flights, decoding abbreviations and transforming raw API data into structured CSV format for easier analysis."
)]
pub struct Cli {
    /// Input jsom file
    pub file: Option<String>,

    /// output CSV
    #[arg(long)]
    pub output: Option<String>,
}

// commands definition
#[derive(Subcommand, Debug)]
pub enum Commands {
    Convert {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    Decode {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(long, value_name = "OUTFILE")]
        output: Option<PathBuf>,
    },
    Inspect {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    Credits,
}

//empty main for now :)
fn main() -> Result<()> {
    Ok(())
}
