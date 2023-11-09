mod from_le_bytes;
mod to_le_bytes;

use from_le_bytes::from_le_bytes;
use to_le_bytes::to_le_bytes;

#[proc_macro_derive(FromLeBytes)]
pub fn derive_from_le_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_le_bytes(input)
}

#[proc_macro_derive(ToLeBytes)]
pub fn derive_to_le_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    to_le_bytes(input)
}
