# Smash

Simple precompile fuzzer with geth bindings.

### To add a target to this repo:

- Make a new file in `src/targets/`
- Implement generation for the precompile input
- Implement the `Target` trait on a new struct

See `src/targets/mod.rs` for an example of the Identity precompile

### To run a target in this repo:

- Make a binary in `bin/`
- Add the new binary to `Cargo.toml`
- `cargo run --bin YOUR_BINARY_NAME`

Check out `bin/identity.rs` for an example. Try it with
`cargo run --bin identity `