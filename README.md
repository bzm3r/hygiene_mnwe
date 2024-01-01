# What should happen

All cases should produce:

```txt
hello world!
```

# What will happen

## Overall:

- [ ] fail: `cargo run -p user`
- [ ] fail: `cargo run -p user --features wrapper`
- [x] ok: `cargo run -p container`
- [x] ok: `cargo run -p container --features wrapper`

 ## Details
#### ✗ `cargo run -p user`

Will produce:

```txt
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

#### ✗ `cargo run -p user --features wrapper`

Will produce:

```txt
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

#### ✓ `cargo run -p container`

Will produce:

```txt
```

#### ✓ `cargo run -p container --features wrapper`

Will produce:

```txt
hello world!
```

