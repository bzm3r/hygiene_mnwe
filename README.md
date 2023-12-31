# What should happen

All cases should produce:

```txt
hello world!
```

# What will happen

Overall:
    - FAIL: `cargo run -p user`
    - FAIL: `cargo run -p user --features wrapper`
    - **SUCCESS**: `cargo run -p container`
    - FAIL: `cargo run -p container --features wrapper`

#### `cargo run -p user`

Will produce:

```txt
   Compiling container v0.1.0 (/home/bzm3r/hygiene_mnwe/container)
   Compiling indirector v0.1.0 (/home/bzm3r/hygiene_mnwe/indirector)
   Compiling user v0.1.0 (/home/bzm3r/hygiene_mnwe/user)
error[E0433]: failed to resolve: could not find `container` in the list of imported crates
 --> user/src/main.rs:3:5
  |
3 |     indirector::container::target!();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `container` in the list of imported crates
  |
  = note: this error originates in the macro `indirector::container::target` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0433`.
error: could not compile `user` (bin "user") due to previous error
```

#### `cargo run -p user --features wrapper`

Will produce:

```txt
error[E0433]: failed to resolve: could not find `container` in the list of imported crates
 --> container/src/lib.rs:6:5
  |
6 |     target!();
  |     ^^^^^^^^^ could not find `container` in the list of imported crates
  |
  = note: this error originates in the macro `target` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this module
  |
2 + use crate::hello_world;
  |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `container` (lib) due to previous error
```

#### `cargo run -p container`

Will produce:

```txt
hello world!
```

#### `cargo run -p container --features wrapper`

Will produce:

```txt
error[E0433]: failed to resolve: could not find `container` in the list of imported crates
 --> container/src/lib.rs:6:5
  |
6 |     target!();
  |     ^^^^^^^^^ could not find `container` in the list of imported crates
  |
  = note: this error originates in the macro `target` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this module
  |
2 + use crate::hello_world;
  |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `container` (lib) due to previous error
```

