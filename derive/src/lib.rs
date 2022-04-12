mod des;
mod ser;
mod crc8;
mod crc16;
mod crc_ctor;

use crc_ctor::CrcCtorReceiver;
use crc8::Crc8Receiver;
use crc16::Crc16Receiver;
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

#[proc_macro_derive(BitumCrc8, attributes(field_names))]
pub fn derive_crc8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Crc8Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}

#[proc_macro_derive(BitumCrc16, attributes(field_names))]
pub fn derive_crc16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Crc16Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}