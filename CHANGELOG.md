# Changelog

## v0.2.0

Changed:
- Removed the `f32` and `f64` features (publically)
- Split the crate in two: `concave_hull` for `f32` precision and `concave_hull_f64` for `f64` precision

Improved:
- Significant performance improvements for datasets larger than roughly 50 points. Performance is worse for very small point clouds, but the small point count means that they finish very quickly anyway.

## v0.1.2

Changed:
- Minor documentation updates

## v0.1.1

Changed:
- Minor documentation updates

## v0.1.0

Initial version