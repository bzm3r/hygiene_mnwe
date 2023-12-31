pub mod hello_world;
pub use payload::target;

#[cfg(feature = "wrapper")]
pub fn wrapper() {
    target!();
}
