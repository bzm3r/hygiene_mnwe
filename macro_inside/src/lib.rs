pub mod hello_world;
pub use goal::inner_macro;

#[cfg(feature = "wrapper")]
pub fn wrapper() {
    inner_macro!();
}
