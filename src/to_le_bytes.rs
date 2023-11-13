use crate::add_trait_bounds;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields};

pub fn to_le_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: ToLeBytes` to every type parameter T.
    let generics = add_trait_bounds(input.generics, &parse_quote!(le_stream::ToLeBytes));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let (iterator_statement, iterator_type) = impl_body(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics le_stream::ToLeBytes for #name #ty_generics #where_clause {
            type Iter = #iterator_type;

            fn to_le_bytes(self) -> <Self as le_stream::ToLeBytes>::Iter {
                #iterator_statement
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

fn impl_body(data: &Data) -> (TokenStream, TokenStream) {
    let mut iterator_type = quote! { std::iter::Empty<u8> };
    let mut iterator_statement = quote! { std::iter::empty::<u8>() };

    match *data {
        Data::Struct(ref structure) => match structure.fields {
            Fields::Named(ref fields) => {
                for field in &fields.named {
                    let item_name = field.ident.clone().expect("struct field has no name");
                    let item_type = &field.ty;

                    iterator_statement.extend(quote! {
                        .chain(<#item_type as le_stream::ToLeBytes>::to_le_bytes(self.#item_name))
                    });

                    iterator_type = quote! {
                        std::iter::Chain<#iterator_type, <#item_type as le_stream::ToLeBytes>::Iter>
                    };
                }

                (iterator_statement, iterator_type)
            }
            Fields::Unit => (iterator_statement, iterator_type),
            Fields::Unnamed(ref fields) => {
                for (index, field) in fields.unnamed.iter().enumerate() {
                    let item_type = &field.ty;

                    iterator_statement.extend(quote! {
                        .chain(<#item_type as le_stream::ToLeBytes>::to_le_bytes(self.#index))
                    });

                    iterator_type = quote! {
                        std::iter::Chain<#iterator_type, <#item_type as le_stream::ToLeBytes>::Iter>
                    };
                }

                (iterator_statement, iterator_type)
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
