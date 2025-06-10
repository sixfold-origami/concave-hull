# concave-hull

`concave-hull` is an implementation of the [gift opening concave hull algorithm](https://deeplearning.lipingyang.org/wp-content/uploads/2019/07/Project-10-report_Implementation-of-a-fast-and-efficient-concave-hull-algorithm.pdf), written in Rust.

![Image: A point cloud roughly in the shape of a question mark, with a concave hull wrapping it fairly closely](fig_1.png)

The top level export is a function called `concave_hull`.
See the docs for that function for details on usage, or check the example at `examples/basic.rs`

## Choosing the concavity parameter

Concave hulls are a somewhat subjective thing.
While it's possible to generate a concave hull which minimizes the area of the final polygon, this is often undesirable, as it leads to very crinkly shapes.
In general, you should pick a concavity parameter which produces "desirable" results on your datasets, whatever that means for your application.
Here is some guidance:
- The concavity parameter ranges from zero to positive infinity
- `0` produces a maximally crinkly shape
- `+inf` prevents any concavity, returning the convex hull of the point cloud
- `40` is usually a good starting point

## Features

This crate has one feature, `benches`, which is only used for benchmarks.
End users of this library should never enable it.

## The CLI Crate

In the `cli` folder is a small CLI that exposes the library's functionality.

Basic usage:
```
cargo run -p cli --release -- 50 ./test_data/question_mark.csv -i ./output.png
```
will generate the above question mark image.
The slight gradient on the hull shows the winding: the first edges are fully red, then they fade to pink.

For more information:
```
cargo run -p cli --release -- --help
```

## Testing

Various point clouds can be found in `test_data`, with different shapes, sizes, and properties.
These are used for unit tests and benchmarks.
