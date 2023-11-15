use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub target: PathBuf,

    /// Number of times to greet
    #[arg(short, long)]
    pub output: PathBuf,
}
