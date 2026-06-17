use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IrNode)]
pub fn derive_ir_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Implement IrNodeKind (separate from IrNode, so user can still impl IrNode)
    let expanded = quote! {
        impl ::asuka::runtime::IrNodeKind for #name {
            fn kind(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };
    
    TokenStream::from(expanded)
}
