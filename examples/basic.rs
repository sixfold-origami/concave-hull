//! Basic usage example
//!
//! TODO: Make this module doc better

use concave_hull::{Point, concave_hull};

/// The points from test_data/polygon.csv
const DATA: [Point; 24] = [
    Point::new(141., 408.),
    Point::new(160., 400.),
    Point::new(177., 430.),
    Point::new(151., 442.),
    Point::new(155., 425.),
    Point::new(134., 430.),
    Point::new(126., 447.),
    Point::new(139., 466.),
    Point::new(160., 471.),
    Point::new(167., 447.),
    Point::new(182., 466.),
    Point::new(192., 442.),
    Point::new(187., 413.),
    Point::new(173., 403.),
    Point::new(165., 430.),
    Point::new(171., 430.),
    Point::new(177., 437.),
    Point::new(175., 443.),
    Point::new(172., 444.),
    Point::new(163., 448.),
    Point::new(156., 447.),
    Point::new(153., 438.),
    Point::new(154., 431.),
    Point::new(160., 428.),
];

pub fn main() {
    let hull = concave_hull(&DATA, 40.);

    println!("{hull:?}");
}
