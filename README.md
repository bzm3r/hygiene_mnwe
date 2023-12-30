# What should happen

All cases should produce:

```txt
hello world!
```

# What will happen

Overall:
    - FAIL: `cargo run -p user`
    - FAIL: `cargo run -p user --features wrapper`
    - **SUCCESS**: `cargo run -p macro_inside`
    - FAIL: `cargo run -p macro_inside --features wrapper`

#### `cargo run -p user`

Will produce:

```txt
error[E0433]: failed to resolve: could not find `macro_inside` in the list of imported crates
 --> user/src/main.rs:3:5
  |
3 |     indirector::macro_inside::inner_macro!();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `macro_inside` in the list of imported crates
  |
  = note: this error originates in the macro `indirector::macro_inside::inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0433`.
error: could not compile `user` (bin "user") due to previous error
```

#### `cargo run -p user --features wrapper`

Will produce:

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

For more information about this error, try `rustc --explain E0433`.
error: could not compile `macro_inside` (lib) due to previous error
```

#### `cargo run -p macro_inside`

Will produce:

```txt
hello world!
```

#### `cargo run -p macro_inside --features wrapper`

Will produce:

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

For more information about this error, try `rustc --explain E0433`.
error: could not compile `macro_inside` (lib) due to previous error
```

