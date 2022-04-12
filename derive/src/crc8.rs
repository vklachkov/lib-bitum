use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, quote_spanned};
use syn::{Generics, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
pub(crate) struct Crc8Receiver {
    ident: Ident,
    generics: Generics,
    data: Data<(), ReceiverField>,
}

impl Crc8Receiver {
    fn fields_to_emit(&self) -> Vec<(&Ident, &Type)> {
        self.data
            .as_ref()
            .take_struct()
            .expect("FieldNames only supports named structs")
            .into_iter()
            .map(|field| (field.ident(), field.ftype()))
            .collect()
    }
}

impl ToTokens for Crc8Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.fields_to_emit();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics BitumCrc8 for #ident #ty_generics #where_clause {
                fn crc(&self, crc: u8, table: &[u8; 256]) -> u8 {
                    table[(crc ^ 0) as usize]
                }
            }
        })
    }
}

#[derive(FromField)]
#[darling(attributes(field_names))]
struct ReceiverField {
    ident: Option<Ident>,
    ty: Type,
}

impl ReceiverField {
    fn ident(&self) -> &Ident {
        self.ident
            .as_ref()
            .expect("FieldNames only supports named fields")
    }

    fn ftype(&self) -> &Type {
        &self.ty
    }
}