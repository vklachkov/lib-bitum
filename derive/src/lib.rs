use std::path::Path;
use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, quote_spanned};
use syn::{Generics, Ident, Type};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BitumDeserialize, attributes(field_names))]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
pub(crate) struct Receiver {
    ident: Ident,
    generics: Generics,
    data: Data<(), ReceiverField>,
}

impl Receiver {
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

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.fields_to_emit();

        let fields_extract: Vec<TokenStream> = fields
            .iter()
            .map(|(ident, ftype)| {
                match ftype {
                    Type::Path(type_path) => {
                        let t = type_path.path.segments.last().unwrap().ident.to_string();
                        if t == "Option" {
                            let t = extract_type_from_option(&ftype).unwrap();

                            quote_spanned! { ident.span() =>
                                let (#ident, pos) = if *flags.#ident {
                                    let r = deserialize_at::<#t, _>(data, pos);
                                    (Some(r.0), r.1)
                                } else {
                                    (None, pos)
                                };
                            }
                        } else {
                            quote_spanned! { ident.span() =>
                                let (#ident, pos) = deserialize_at::<#ftype, _>(data, pos);
                            }
                        }
                    },
                    _ => todo!()
                }
            })
            .collect();
        
        let fields: Vec<&Ident> = fields
            .iter()
            .map(|(ident, _)| {
                *ident
            })
            .collect();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics BitumDeserializeOwned for #ident #ty_generics #where_clause {
                fn deserialize_at<const N: usize>(data: &[u8; N], pos: BitPosition) -> (Self, BitPosition) {                    
                    fn deserialize_at<T, const N: usize>(data: &[u8; N], pos: BitPosition) -> (T, BitPosition)
                        where T: BitumDeserializeOwned 
                    {
                        T::deserialize_at(data, pos)
                    }

                    #(#fields_extract)*

                    (
                        #ident {
                            #(#fields),*
                        },
                        pos
                    )
                }
            }
        })
    }
}

fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });

        path.segments.last()
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
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