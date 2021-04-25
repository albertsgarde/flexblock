extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
//use quote::quote;

mod expansion;


#[proc_macro_derive(ShaderId, attributes(extensionless_path,name,is_compute))]
pub fn derive_shader_id(input : TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    return expansion::expand_shader_id(input);

}
