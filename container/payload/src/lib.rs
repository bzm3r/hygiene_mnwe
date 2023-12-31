use quote::quote;

#[proc_macro]
pub fn target(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!(::container::hello_world::print()).into()
}
