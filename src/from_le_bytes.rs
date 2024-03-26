use crate::add_trait_bounds;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields};

pub fn from_le_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics, &parse_quote!(le_stream::FromLeBytes));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = impl_body(&input.data);
    let expanded = quote! {
        impl #impl_generics le_stream::FromLeBytes for #name #ty_generics #where_clause {
            fn from_le_bytes<T>(bytes: &mut T) -> ::le_stream::Result<Self>
            where
                T: ::std::iter::Iterator<Item = u8>
            {
                #body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn impl_body(data: &Data) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut constructor_fields = TokenStream::new();

    match *data {
        Data::Struct(ref structure) => match structure.fields {
            Fields::Named(ref fields) => {
                for field in &fields.named {
                    let item_name = field.ident.clone().expect("struct field has no name");
                    let item_type = &field.ty;

                    tokens.extend(quote! {
                        let #item_name = <#item_type as ::le_stream::FromLeBytes>::from_le_bytes(bytes)?;
                    });

                    constructor_fields.extend(quote! {
                        #item_name,
                    });
                }

                tokens.extend(quote! { Ok(Self { #constructor_fields }) });
                tokens
            }
            Fields::Unit => {
                quote! { Ok( Self {} ) }
            }
            Fields::Unnamed(ref fields) => {
                for (index, field) in fields.unnamed.iter().enumerate() {
                    let item_name = format!("field_{index}");
                    let item_type = &field.ty;

                    tokens.extend(quote! {
                        let #item_name = <#item_type as ::le_stream::FromLeBytes>::from_le_bytes(bytes)?;
                    });

                    constructor_fields.extend(quote! {
                        #item_name,
                    });
                }

                tokens.extend(quote! { Ok(Self(#constructor_fields)) });
                tokens
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
