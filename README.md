# Smash

Simple precompile fuzzer with geth and celo bindings.

This code is largely based on
[Shamatar's algebraic fuzzer](https://github.com/shamatar/algebraic_fuzzer/).
His code is used with permission, pending FOSS licensing.

### Requirements

- working `cargo` and `go` installations

### Status

- identity: working
- sha2: working
- cip20: working
- eip2537: TODO
- eip2539: TODO

### To add a target to this repo:

- Make a new file in `src/targets/`
- Implement randomized generation for the precompile input
- Implement the `Target` trait on a new struct, with 1 or more experimental
    run
    - **Note**: This library expects `Target::generate()` to always produce
    valid input. Thus support for lain `#[derive()]` macros is very limited.
- Optionally: implement `ProduceInvalid` to test invalid inputs for panics
    - This enables `Fuzzer::run_invalid()` and `Fuzzer::run_mixed()`
- Optionally: implement `TargetWithControl` to compare output to a control
    output
    - This enables `Fuzzer::run_against_control()`

See `src/targets/identity.rs` for an example of the Identity precompile

### To run a target in this repo:

- Make a binary in `bin/`
- Add the new binary to `Cargo.toml`
- `cargo run --release --bin YOUR_BINARY_NAME`

Check out `bin/identity.rs` for an example. Check out the CLI options with
`cargo run --bin identity -- --help`

### To use this on other geth implementations

- Make new bindings
- Use the new bindings in your `Target::run_experimental()`

See `call_geth/` for an example of calling a go binary via stdin.

Unfortunately, due to limitations of go's module systems, the bound golang
code must be in a tagged commit (or local). Branches and commit hashes are not
permitted. Use the replace directive to specify a repo or local directory.