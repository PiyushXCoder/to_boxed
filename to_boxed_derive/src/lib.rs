use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ToBoxed)]
pub fn to_boxed_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse(input).unwrap();
    impl_to_boxed(&ast)
}

fn impl_to_boxed(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let token_stream = quote! {
        impl ToBoxed for #name {
            fn to_boxed(self) -> Box<Self> {
                Box::new(self)
            }
        }
    };
    token_stream.into()
}
