fn main() {
    #[cfg(feature = "original")]
    indirector::container::target!(indirector::container);
    #[cfg(feature = "wrapper")]
    indirector::container::wrapper();
}
