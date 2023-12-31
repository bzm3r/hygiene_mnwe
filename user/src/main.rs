fn main() {
    #[cfg(feature = "original")]
    indirector::container::target!();
    #[cfg(feature = "wrapper")]
    indirector::container::wrapper();
}
