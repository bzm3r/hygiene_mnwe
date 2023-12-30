use indirection::macro_container::{inner_macro, wrapper};

#[cfg(feature = "original")]
fn main() {
    inner_macro!()
}

#[cfg(feature = "wrapper")]
fn main() {
    wrapper()
}
