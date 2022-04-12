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
                if ident.to_string().starts_with("crc") {
                    quote_spanned! { ident.span() =>

                    }
                } else {
                    quote_spanned! { ident.span() =>
                        #ident: #t,
                    }
                }
            })
            .collect();

        enum CrcType { CRC8, CRC16 }

        let crc_field_name = fields.iter().find(|(ident, _)| { ident.to_string().starts_with("crc") });
        let crc_type = match crc_field_name {
            Some((i, _)) => {
                match i.to_string().as_str() {
                    "crc8" => CrcType::CRC8,
                    "crc16" => CrcType::CRC16,
                    _ => {
                        //println!("UUU {}", i.to_string().as_str());
                        //CrcType::CRC8
                        panic!("Invalid crc field name")
                    }
                }
            },
            None => {
                //println!("UUU");
                //CrcType::CRC8
                panic!("Crc not found")
            },
        };

        let crc_calc: Vec<TokenStream> = fields
            .iter()
            .map(|(ident, t)| {
                if ident.to_string().starts_with("crc") {
                    quote_spanned! { ident.span() =>
                    
                    }
                } else if ident.to_string() == "flags" {
                    quote_spanned! { ident.span() =>
                        crc = #ident.crc(crc, table);
                    }
                } else {
                    let crc_calc = match crc_type {
                        CrcType::CRC8 => quote_spanned! { ident.span() =>
                            table[(crc ^ b) as usize]
                        },
                        CrcType::CRC16 => quote_spanned! { ident.span() =>
                            (crc << 8) ^ table[((crc >> 8) ^ (*b as u16)) as usize]
                        },
                    };
                    
                    quote_spanned! { ident.span() =>
                        {
                            let bytes = unsafe {
                                let len = core::mem::size_of::<#t>();
                                let ptr = &#ident as *const #t as *const u8;
                                core::slice::from_raw_parts(ptr, len)
                            };

                            for b in bytes {
                                crc = #crc_calc;
                            }
                        }
                    }
                }
            })
            .collect();

        let args: Vec<TokenStream> = fields
            .iter()
            .map(|(ident, _)| {
                if ident.to_string().starts_with("crc") {
                    quote_spanned! { ident.span() =>
                    
                    }
                } else {
                    quote_spanned! { ident.span() =>
                        #ident,
                    }
                }
            })
            .collect();

        let crc_t = match crc_type {
            CrcType::CRC8 => quote_spanned! { ident.span() => u8 },
            CrcType::CRC16 => quote_spanned! { ident.span() => u16 },
        };

        let crc_n = match crc_type {
            CrcType::CRC8 => quote_spanned! { ident.span() => crc8 },
            CrcType::CRC16 => quote_spanned! { ident.span() => crc16 },
        };

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn new_with_checksum(#(#params)* table: &[#crc_t; 256]) -> Self {
                    let #crc_n = {
                        let mut crc: #crc_t = #crc_t::MAX;
                        #(#crc_calc)*
                        crc
                    };

                    Self {
                        #(#args)*
                        #crc_n
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