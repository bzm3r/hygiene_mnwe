#[cfg(feature = "original")]
use indirector::macro_inside::inner_macro;

#[cfg(feature = "wrapper")]
use indirector::macro_inside::wrapper;

#[cfg(feature = "original")]
fn main() {
    inner_macro!()
}

#[cfg(feature = "wrapper")]
fn main() {
    wrapper()
}
