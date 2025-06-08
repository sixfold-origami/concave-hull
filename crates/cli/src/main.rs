use std::{fs::File, path::PathBuf};

use anyhow::Ok;
use clap::Parser;
use concave_hull::{Point, concave_hull};
use csv::{ReaderBuilder, Writer};

/// Basic CLI to interface with the concave hull library
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to input CSV file, with an x column and y column (in order)
    input: String,

    /// Concavity parameter to use
    concavity: f32,

    /// Path to output hull points to
    #[arg(short, long, default_value = "output.csv")]
    output: String,

    /// Whether the input CSV has headers
    #[arg(short, long, default_value_t = true)]
    headers: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let input = PathBuf::from(args.input);
    let output = PathBuf::from(args.output);

    println!(
        "Generating concave hull for {} [concavity: {}]",
        input.display(),
        args.concavity
    );

    // Read input points
    let f = File::open(input)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(args.headers)
        .from_reader(f);

    let in_points = reader
        .records()
        .map(|r| {
            let r = r?;
            let x = r[0].parse()?;
            let y = r[1].parse()?;

            Ok(Point::new(x, y))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Generate hull
    let out_points = concave_hull(&in_points);

    // Output
    let mut writer = Writer::from_path(output)?;
    for p in out_points {
        writer.write_record(&[p.x.to_string(), p.y.to_string()])?
    }

    Ok(())
}
