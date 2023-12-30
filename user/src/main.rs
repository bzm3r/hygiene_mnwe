fn main() {
    #[cfg(feature = "original")]
    indirector::macro_inside::inner_macro!();
    #[cfg(feature = "wrapper")]
    indirector::macro_inside::wrapper();
}
