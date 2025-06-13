# TODO

- Compat features for crates other than nalgebra (`glam`, `geo`, that one compat crate (`mint`?))
- Handle f32/f64 versions of `parry` (we should probably have two mirrored crates, like `parry`/`nalgebra` do)
- Tests
- Compare to `geo`'s impl
- Use a non-zero f32 for the concavity parameter?
- Make the two point degenerate case not panic, and add a test for it
- Check what happens if you have duplicate points
