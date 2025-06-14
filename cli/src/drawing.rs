use concave_hull::f32::{
    Point,
    parry2d::{
        bounding_volume::{BoundingVolume, details::local_point_cloud_aabb},
        math::Vector,
    },
};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_line_segment_mut},
    image::{Rgb, RgbImage},
    pixelops::interpolate,
};

/// Padding added to each side of the image, so that points aren't right up against the edge of the canvas
///
/// Stored as an f32 because it makes it slightly more convenient
const IMG_PADDING: f32 = 10.;

const POINT_COLOR: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
const FULL_SEGMENT_COLOR: Rgb<u8> = Rgb([255u8, 0u8, 0u8]);
const FADED_SEGMENT_COLOR: Rgb<u8> = Rgb([255u8, 200u8, 200u8]);

pub fn draw_points_and_hull(mut points: Vec<Point>, mut hull: Vec<Point>, debug: bool) -> RgbImage {
    // Note: coordinates are mirrored about the y axis before being drawn,
    // since imageproc uses the standard image coordinate space (y-down),
    // but parry (and, by extension, this crate) use the standard mathematical coordinate space (y-up).
    // This mirroring in the rendering steps keeps things consistent,
    // and ensures that the gradient for winding order actually goes in the correct direction.
    if !debug {
        points
            .iter_mut()
            .for_each(|p| *p = p.coords.component_mul(&Vector::new(1.0, -1.0)).into());
        hull.iter_mut()
            .for_each(|p| *p = p.coords.component_mul(&Vector::new(1.0, -1.0)).into());
    }

    let mut aabb = local_point_cloud_aabb(&points).loosened(IMG_PADDING);
    if debug {
        aabb.mins = Point::origin();
    }
    let point_size = (aabb.extents().max() / 250.).max(2.) as i32;

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

    for i in 0..hull.len() {
        let j = (i + 1) % hull.len();
        let a = hull[i] - aabb.mins;
        let b = hull[j] - aabb.mins;

        // Interpolate from full to faded as we go around
        let color = interpolate(
            FADED_SEGMENT_COLOR,
            FULL_SEGMENT_COLOR,
            i as f32 / hull.len() as f32,
        );

        draw_filled_circle_mut(&mut image, (a.x as i32, a.y as i32), point_size, color);
        draw_line_segment_mut(&mut image, (a.x, a.y), (b.x, b.y), color);
    }

    image
}
