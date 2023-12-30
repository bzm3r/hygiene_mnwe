`cargo run` should produce the error:

```
error[E0433]: failed to resolve: could not find `macro_container` in the list of imported crates
 --> src/main.rs:5:5
  |
5 |     inner_macro!()
  |     ^^^^^^^^^^^^^^ could not find `macro_container` in the list of imported crates
  |
  = note: this error originates in the macro `inner_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing this crate
  |
1 + use indirection::macro_container;
  |

warning: unused import: `indirection::macro_container`
 --> src/main.rs:1:5
  |
1 | use indirection::macro_container;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
```
