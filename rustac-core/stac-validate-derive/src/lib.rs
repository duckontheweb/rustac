extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Validation)]
pub fn validation_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_validation(&ast)
}

fn impl_validation(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Validate for #name {
            fn get_stac_version(&self) -> &Version { &self.stac_version }
            fn get_type(&self) -> &String { &self.type_ }
        }
    };
    gen.into()
}
