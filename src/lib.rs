mod from_le_bytes;
mod to_le_bytes;

use from_le_bytes::from_le_bytes;
use syn::{GenericParam, Generics, TypeParamBound};
use to_le_bytes::to_le_bytes;

#[proc_macro_derive(FromLeBytes)]
pub fn derive_from_le_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_le_bytes(input)
}

#[proc_macro_derive(ToLeBytes)]
pub fn derive_to_le_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_le_bytes(input)
}

fn add_trait_bounds(mut generics: Generics, trait_name: &TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(trait_name.clone());
        }
    }
    generics
}
