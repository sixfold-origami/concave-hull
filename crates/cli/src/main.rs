use std::path::PathBuf;

use clap::Parser;

/// Basic CLI to interface with the concave hull library
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to input file
    pub input: String,

    /// Concavity parameter to use
    pub concavity: f32,

    /// Path to output polyline points to
    #[arg(short, long, default_value = "output.csv")]
    pub output: String,
}

fn main() {
    let args = Cli::parse();
    let input = PathBuf::from(args.input);
    let _output = PathBuf::from(args.output);

    println!(
        "Generating concave hull for {} [concavity: {}]",
        input.display(),
        args.concavity
    );
}
