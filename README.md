Difference from [`main`](https://github.com/bzm3r/hygiene_mnwe/tree/main) branch: addresses possible error in original `main` branch of not explicitly exporting `message` function from the `hello_world` module of `macro_container`. Also, in order to test whether `macro_container` can properly use `inner`, `macro_container` now exports a function `wrapper` which calls `inner_macro`.

To try out this funciton, introduced a new feature called `"wrapper"` on the primary binary. Neither the original `main` (`"default"` feature) or the `"wrapper"`'s `main` work.

`cargo run` should produce:

```
error[E0433]: failed to resolve: could not find `macro_container` in the list of imported crates
 --> macro_container/src/lib.rs:6:5
  |
6 |     inner_macro!();
  |     ^^^^^^^^^^^^^^ could not find `macro_container` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
```

`cargo run --features wrapper` should produce:

```
error[E0433]: failed to resolve: could not find `macro_container` in the list of imported crates
 --> macro_container/src/lib.rs:6:5
  |
6 |     inner_macro!();
  |     ^^^^^^^^^^^^^^ could not find `macro_container` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
```

Note that the errors are identical in this case.

Also added a `main` function to `macro_container`, to see if it can call `inner_macro` successfully (it does not).

`cargo run --manifest-path ./macro_container/Cargo.toml` should produce:

```
error[E0433]: failed to resolve: could not find `macro_container` in the list of imported crates
 --> src/lib.rs:6:5
  |
6 |     inner_macro!();
  |     ^^^^^^^^^^^^^^ could not find `macro_container` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
```
