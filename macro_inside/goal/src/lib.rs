use quote::quote;

#[proc_macro]
pub fn inner_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!(::macro_inside::hello_world::print()).into()
}
