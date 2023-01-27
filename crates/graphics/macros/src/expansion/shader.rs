use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;

pub fn expand_shader_id(input: DeriveInput) -> TokenStream {
    // TODO: Make it so two shaders cannot have same name / extensionless path

    let name = input.ident;
    let mut names = Vec::new();
    let mut extensionless_paths = Vec::new();
    let mut are_compute = Vec::new();
    let mut variants = Vec::new();
    if let Data::Enum(data_enum) = input.data {
        for variant in data_enum.variants.into_iter() {
            variants.push(variant.ident);
            let attrs = variant.attrs;
            let mut has_name = false;
            let mut has_exp = false;
            let mut has_comp = false;
            for attr in attrs {
                match &attr.path.get_ident().unwrap().to_string()[..] {
                    "name" => {
                        if has_name {
                            panic!("A shader identifier has been given two names!");
                        }
                        names.push(attr.tokens);
                        has_name = true;
                    }
                    "extensionless_path" => {
                        if has_exp {
                            panic!("A shader identifier has been given two extensionless paths!");
                        }
                        extensionless_paths.push(attr.tokens);
                        has_exp = true;
                    }
                    "is_compute" => {
                        if has_comp {
                            panic!("A shader identifier has been given two is_compute settings!");
                        }
                        are_compute.push(attr.tokens);
                        has_comp = true;
                    }
                    _ => {}
                }
            }
        }
    }

    if variants.len() > names.len() {
        panic!("Not all shader identifiers have been given names!");
    }
    if variants.len() > extensionless_paths.len() {
        panic!("Not all shader identifiers have been given extensionless paths!");
    }
    if variants.len() > are_compute.len() {
        panic!("Not all shader identifiers have been given is_compute settings!");
    }

    let range = 0..variants.len();
    let exp = range.map(|x| {
        let id = &variants[x];
        let res = &extensionless_paths[x];
        quote! {
            #name::#id => #res,
        }
    });
    let range = 0..variants.len();
    let nms = range.map(|x| {
        let id = &variants[x];
        let res = &names[x];
        quote! {
            #name::#id => #res,
        }
    });
    let range = 0..variants.len();
    let cmps = range.map(|x| {
        let id = &variants[x];
        let res = &are_compute[x];
        quote! {
            #name::#id => #res,
        }
    });

    let k = quote! {
        #[automatically_derived]
        impl #name {
            #[allow(unused_parens)]
            pub fn extensionless_path(&self) -> &'static str {
                match self {
                    #(#exp)*
                }
            }
            #[allow(unused_parens)]
            pub fn name(&self) -> &'static str {
                match self {
                    #(#nms)*
                }
            }
            #[allow(unused_parens)]
            pub fn is_compute(&self) -> bool {
                match self {
                    #(#cmps)*
                }
            }
        }
    };

    k.into()
}
