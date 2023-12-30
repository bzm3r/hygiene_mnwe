# What should happen

```txt
hello world!
```

# What will happen

`cargo run --bin user` should produce:

```txt
error[E0433]: failed to resolve: could not find `macro_inside` in the list of imported crates
 --> user/src/main.rs:9:5
  |
9 |     inner_macro!()
  |     ^^^^^^^^^^^^^^ could not find `macro_inside` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
```

`cargo run --bin user --features wrapper` should produce:

```txt
 --> macro_inside/src/lib.rs:6:5
  |
6 |     inner_macro!();
  |     ^^^^^^^^^^^^^^ could not find `macro_inside` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this module
  |
2 + use crate::hello_world;
  |
```

`cargo run --bin macro_inside` should produce:

```txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/macro_inside`
hello world!
```

`cargo run --bin macro_inside --features wrapper` should produce:

```txt
error[E0433]: failed to resolve: could not find `macro_inside` in the list of imported crates
 --> macro_inside/src/lib.rs:6:5
  |
6 |     inner_macro!();
  |     ^^^^^^^^^^^^^^ could not find `macro_inside` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this module
  |
2 + use crate::hello_world;
  |
```

# Aside: difference from the original main (if you saw it)

Difference between this commit and the original [`main`](https://github.com/bzm3r/hygiene_mnwe/tree/71ecc4b2f09b7b8cfe12052167996a9dbc9bcb5f) branch: addresses possible error in original `main` branch of not explicitly exporting `print` function from the `hello_world` module of `macro_inside`. Also, in order to test whether `macro_inside` can properly use `inner`, `macro_inside` now exports a function `wrapper` which calls `inner_macro`.

To try conditionally try a code path using this function, a new feature called `"wrapper"` is introduced.

Also created a `main` function on `macro_inside`, which allows testing whether it can use `inner_macro!` successfully (it can).

Finally, created a separate crate for the testing binary called `user`.
