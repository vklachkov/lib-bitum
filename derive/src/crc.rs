use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, quote_spanned};
use syn::{Generics, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
pub(crate) struct CrcCtorReceiver {
    ident: Ident,
    generics: Generics,
    data: Data<(), ReceiverField>,
}

impl CrcCtorReceiver {
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

impl ToTokens for CrcCtorReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.fields_to_emit();
        
        let params: Vec<TokenStream> = fields
            .iter()
            .map(|(ident, t)| {
                if ident.to_string() == "crc" {
                    quote_spanned! { ident.span() =>

                    }
                } else {
                    quote_spanned! { ident.span() =>
                        #ident: #t,
                    }
                }
            })
            .collect();

        let crc_calc: Vec<TokenStream> = fields
            .iter()
            .map(|(ident, t)| {
                if ident.to_string() == "crc" {
                    quote_spanned! { ident.span() =>
                    
                    }
                } else {
                    quote_spanned! { ident.span() =>
                        {
                            let bytes = unsafe {
                                let len = core::mem::size_of::<#t>();
                                let ptr = &#ident as *const #t as *const u8;
                                core::slice::from_raw_parts(ptr, len)
                            };

                            for b in bytes {
                                crc = crc_table[(crc ^ b) as usize];
                            }
                        }
                    }
                }
            })
            .collect();

        let args: Vec<TokenStream> = fields
            .iter()
            .map(|(ident, t)| {
                if ident.to_string() == "crc" {
                    quote_spanned! { ident.span() =>
                    
                    }
                } else {
                    quote_spanned! { ident.span() =>
                        #ident,
                    }
                }
            })
            .collect();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                fn new_with_checksum(#(#params)* crc_table: &[u8; 256]) -> Self {
                    let crc = {
                        let mut crc: u8 = 0xFF;

                        #(#crc_calc)*
                        
                        crc
                    };

                    Self {
                        #(#args)*
                        crc    
                    }
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