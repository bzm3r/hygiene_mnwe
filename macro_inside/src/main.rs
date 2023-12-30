#[cfg(feature = "original")]
use goal::inner_macro;

#[cfg(feature = "wrapper")]
use crate::wrapper;

#[cfg(feature = "original")]
fn main() {
    inner_macro!()
}

#[cfg(feature = "wrapper")]
fn main() {
    crate::wrapper()
}
