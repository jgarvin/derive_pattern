use inflector::Inflector;
use proc_macro2::Span;
use quote::quote;
use syn::{Fields, FieldsNamed, Ident, ItemStruct};

#[proc_macro_derive(Pattern)]
pub fn derive_pattern(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let structure: ItemStruct = syn::parse2(input).expect("derive(Pattern) only works on structs.");

    let name = structure.ident;

    let macro_name = Ident::new(
        &(name.to_string().to_snake_case() + "_pattern"),
        name.span(),
    );

    match &structure.fields {
        Fields::Named(FieldsNamed {
            brace_token: _,
            named,
        }) => {
            let mut field_names: Vec<Ident> = named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().clone())
                .collect();
            for field_name in &mut field_names {
                field_name.set_span(Span::call_site());
            }
            let output = quote! {
                macro_rules! #macro_name {
                    () => {
                        #name{ #(#field_names),* }
                    }
                }
            };
            output.into()
        }
        _ => panic!("derive(Pattern) only supports structs with named fields"),
    }
}
