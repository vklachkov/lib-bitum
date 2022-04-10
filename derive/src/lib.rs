mod des;
mod ser;
mod crc;

use crc::CrcCtorReceiver;
use darling::FromDeriveInput;
use des::DeserializeReceiver;
use quote::quote;
use ser::SerializeReceiver;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BitumDeserialize, attributes(field_names))]
pub fn derive_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    DeserializeReceiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

#[proc_macro_derive(BitumSerialize, attributes(field_names))]
pub fn derive_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    SerializeReceiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

#[proc_macro_derive(BitumCrcConstructor, attributes(field_names))]
pub fn derive_crc_ctor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    CrcCtorReceiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
