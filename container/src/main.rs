fn main() {
    #[cfg(feature = "original")]
    container::target!(container);
    #[cfg(feature = "wrapper")]
    container::wrapper();
}
