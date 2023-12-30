fn main() {
    #[cfg(feature = "original")]
    macro_inside::inner_macro!();
    #[cfg(feature = "wrapper")]
    macro_inside::wrapper();
}
