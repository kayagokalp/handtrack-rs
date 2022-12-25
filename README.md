# handtrack-rs

This repo provides a Rust API encapsulating the works of https://github.com/victordibia/handtracking. Basically this library encapsulates the required tensorflow interactions and provide a easy-to-use API for simply detecting hands. 

This is very much in WIP and I have only tested this with single images. My aim here to basically be able to detect hands in real-time video just like the js version of this little library at https://github.com/victordibia/handtrack.js

## Example

```rust
// Import the image.
let image = Image::from_file(project_dir).unwrap();

// Construct detection options.
let score_threshold = 0.7f32;
let max_hands = 1;
let detection_opts = DetectionOptions::new(max_hands, score_threshold);

// Run the detection.
let detection = detect(image, detection_opts).unwrap();

let detection_box = &detection[0];
```

As it can be seen from the example above, `detect` function requires an `Image` and `DetectionOptions`. Currently it is possible to specify desired maximum number of hands detected and score threshold for classifying an object as a hand.

## Contribution

Although this is a small library it does have lots of missing features and contributions are more than welcome! As this is very early stage I do not have set contribution guidelines but I have some CI checks in place for just in case which are:

- `clippy` linting
- `cargo fmt` checking
- `Cargo.toml` linting (dependencies must be in alphabetical order etc.)
- `cargo test` check
