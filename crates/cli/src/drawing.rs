use concave_hull::{
    Edge, Point,
    parry2d::bounding_volume::{BoundingVolume, details::local_point_cloud_aabb},
};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_line_segment_mut},
    image::{Rgb, RgbImage},
};

/// Padding added to each side of the image, so that points aren't right up against the edge of the canvas
///
/// Stored as an f32 because it makes it slightly more convenient
const IMG_PADDING: f32 = 10.;

const POINT_COLOR: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
const SEGMENT_COLOR: Rgb<u8> = Rgb([255u8, 0u8, 0u8]);

pub fn draw_points_and_hull(points: &[Point], hull: &[Edge]) -> RgbImage {
    let aabb = local_point_cloud_aabb(points).loosened(IMG_PADDING);
    let point_size = (aabb.extents().max() / 250.).max(2.) as i32;
    // let line_width = (aabb.extents().max() / 500.).max(2.) as u32;

    let mut image = RgbImage::new(aabb.extents().x as u32, aabb.extents().y as u32);

    for point in points {
        let point = point - aabb.mins;
        draw_filled_circle_mut(
            &mut image,
            (point.x as i32, point.y as i32),
            point_size,
            POINT_COLOR,
        );
    }

    for edge in hull {
        let a = edge.segment.a - aabb.mins;
        let b = edge.segment.b - aabb.mins;

        draw_filled_circle_mut(
            &mut image,
            (a.x as i32, a.y as i32),
            point_size,
            SEGMENT_COLOR,
        );

        draw_line_segment_mut(&mut image, (a.x, a.y), (b.x, b.y), SEGMENT_COLOR);
    }

    image
}
