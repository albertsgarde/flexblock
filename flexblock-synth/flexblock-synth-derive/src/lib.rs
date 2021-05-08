use proc_macro::TokenStream;
use proc_macro2::{self, Span, Ident};
use quote::quote;
use syn::{self};

#[proc_macro_attribute]
pub fn module(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    module_inner(item).into()
}

fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

fn module_inner(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: syn::DeriveInput = syn::parse2(item).unwrap();
    let mut gen = quote! {
        #[derive(Clone)]
        #ast
    };
    gen
}

fn impl_operator(ast: &syn::DeriveInput, operator: &Ident, operator_function: &Ident, operator_module: &Ident) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let type_params = generics.type_params();
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl <#(#type_params,)* RHS: crate::modules::Module> std::ops::#operator<RHS> for #name #ty_generics #where_clause {
            type Output = crate::modules::ModuleTemplate<crate::modules::#operator_module<#name #ty_generics, RHS>>;

            fn #operator_function(self, rhs: RHS) -> Self::Output {
                crate::modules::#operator_module::new(self, rhs)
            }
        }
    };
    gen
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn module_for_one_pole_filter() {
        let generated = module_inner(
            quote!{
                pub struct OnePoleFilter<S: Module, C: Module> {
                    source: S,
                    coefficient: C,
                    prev_sample: f32,
                }
            }
            .into(),
        );
        let generated_code = format!("{}", generated);
        let output_path = "assets/tests/one_pole_filter_module_output.rs";
        fs::write(output_path, generated_code.as_bytes()).unwrap();
    }

    #[test]
    fn no_module_generics() {
        let generated = module_inner(
            quote!{
                pub struct Foo {
                    bar: f32,
                }
            }
            .into(),
        );
        let generated_code = format!("{}", generated);
        let output_path = "assets/tests/no_module_generics_output.rs";
        fs::write(output_path, generated_code.as_bytes()).unwrap();
    }
}
