pub mod hello_world;
pub use hello_world::message;
pub use inside::inner_macro;

pub fn wrapper() {
    inner_macro!();
}
