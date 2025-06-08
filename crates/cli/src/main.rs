use std::{fs::File, path::PathBuf};

use anyhow::Ok;
use clap::Parser;
use concave_hull::{Point, concave_hull};
use csv::{ReaderBuilder, Writer};

use crate::drawing::draw_points_and_hull;

mod drawing;

/// Basic CLI to interface with the concave hull library
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Concavity parameter to use
    concavity: f32,

    /// Path to input CSV file, with an x column and y column (in order)
    input: String,

    /// Path to output a CSV of hull points to
    #[arg(short, long)]
    point_output: Option<String>,

    /// Path to output a PNG image of the points and hull to
    #[arg(short, long)]
    img_output: Option<String>,

    /// Whether the input CSV has headers
    #[arg(short, long, default_value_t = false)]
    headers: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let input = PathBuf::from(args.input);
    let point_output = args.point_output.map(|path| PathBuf::from(path));
    let img_output = args.img_output.map(|path| PathBuf::from(path));

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
    let hull = concave_hull(&in_points, args.concavity);

    // Output
    if point_output.is_none() && img_output.is_none() {
        println!("No output provided. Terminating.");
    }

    if let Some(point_output) = point_output {
        println!(
            "Writing concave hull points to {:?}",
            point_output.display()
        );

        let mut writer = Writer::from_path(point_output)?;
        for edge in hull.iter() {
            writer.write_record(&[edge.segment.a.x.to_string(), edge.segment.a.y.to_string()])?
        }
    }

    if let Some(img_output) = img_output {
        println!(
            "Drawing image of points and hull at {:?}",
            img_output.display()
        );

        let image = draw_points_and_hull(&in_points, &hull);
        image.save(img_output)?;
    }

    Ok(())
}
