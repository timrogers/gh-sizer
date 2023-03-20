# Contributing

## Testing

`gh sizer` is tested with:

* unit tests run across macOS, Linux and Windows
* integration tests run against the `gh-sizer` binary in a Linux environment

### Unit tests

To run the unit tests, run `cargo test`.

### Integration tests

Integration tests are part of the `integration_tests` [feature](https://doc.rust-lang.org/cargo/reference/features.html) and are not compiled and run by default. To run them, compile with `cargo build --features integration_tests` and then test with `cargo test --features integration_tests`. In order to run the integration tests, the `gh` CLI must be authenticated - you can authenticate by running `gh auth login` or by setting the `GH_TOKEN` environment variable.

### Working with snapshot tests

This project makes use of snapshot tests powered by [Insta](https://insta.rs/). Snapshots tests use the `insta::assert_yaml_snapshot!` function.

After adding a new snapshot test, run the test and then run `cargo insta review` to accept the recorded snapshot.

To update a record snapshot, delete the corresponding file (`**/snapshots/*.snap`), run the relevant test and then run `cargo insta review` to accept ther recorded snapshot.

## Formatting

Code should be formatted according to the default standards of Cargo's built-in formatter. To auto-format your code, run `cargo fmt`.

## Releasing a new version

To release a new version:

1. Update the `version` declaration in `Cargo.toml`.
2. Create and push a tag with the version number, e.g. `v1.2.3`:

```bash
git tag -a v1.2.3 -m 'v1.2.3'
git push --tags
```

3. Wait for the "Build, test and release" workflow to finish.
4. Update the created release with a description.

The "Build, test and release" workflow will automatically build and upload `amd64` binaries for Linux, macOS and Windows. At this time, 32-bit and ARM binaries are not generated.