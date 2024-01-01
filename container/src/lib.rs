pub mod hello_world;
pub extern crate self as container;
pub use payload::target;

#[cfg(feature = "wrapper")]
pub fn wrapper() {
    target!();
}
