extern crate container as this_container;

fn main() {
    #[cfg(feature = "original")]
    container::target!();
    #[cfg(feature = "wrapper")]
    container::wrapper();
}
