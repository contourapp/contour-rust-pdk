use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse::Parse};
use itertools::Itertools;

#[proc_macro_attribute]
pub fn listener_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("listener_fn expects a public function");
    }

    let name = &function.sig.ident;
    let constness = &function.sig.constness;
    let unsafety = &function.sig.unsafety;
    let generics = &function.sig.generics;
    let inputs = &mut function.sig.inputs;
    let output = &mut function.sig.output;
    let block = &function.block;

    let (input_name, input_ty) = if inputs.len() != 1 {
        panic!("listener_fn expects a function that accepts one parameter");
    } else {
        match inputs.first().unwrap() {
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => match pat.as_ref() {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => (ident, ty),
                _ => panic!("listener_fn expects a function that accepts one parameter"),
            },
            _ => panic!("listener_fn expects a function that accepts one parameter"),
        }
    };

    if name == "main" {
        panic!(
            "listener_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        )
    }

    match output {
        syn::ReturnType::Default => {
            panic!("listener_fn expects a return value, `()` may be used if no output is needed")
        }
        syn::ReturnType::Type(_, t) => {
            if let syn::Type::Path(p) = t.as_ref() {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "FnResult" {
                        panic!("listener_fn expects a function that returns FnResult");
                    }
                } else {
                    panic!("listener_fn expects a function that returns FnResult");
                }
            }
        }
    };
    let command_type = get_type_string(input_ty);
    let name_inner = syn::Ident::new(&format!("{}_inner", name), name.span());

    quote! {
        #constness #unsafety fn #name_inner #generics(#input_name: #input_ty) #output {
            #block
        }

        #[no_mangle]
        pub #constness #unsafety extern "C" fn #name() -> i32 {
            use contour_rust_pdk::extism_pdk;

            let input = extism_pdk::input();

            match input {
                Ok(input) => {
                    let extism_pdk::Json(json): contour_rust_pdk::extism_pdk::Json<serde_json::Value> = input;
                    let generic_input: contour_rust_pdk::io::HandlerInput::<serde_json::Value> = serde_json::from_value(json.clone()).unwrap();

                    if generic_input.command_type == #command_type {
                        let input: contour_rust_pdk::io::HandlerInput::<#input_ty> = serde_json::from_value(json).unwrap();
                        let output = match #name_inner(input.command) {
                            Ok(x) => x,
                            Err(rc) => {
                                let err = format!("{:?}", rc.0);
                                let mut mem = extism_pdk::Memory::from_bytes(&err).unwrap();
                                unsafe {
                                    extism_pdk::extism::error_set(mem.offset());
                                }
                                return rc.1;
                            }
                        };
                        extism_pdk::unwrap!(extism_pdk::output(&output));
                    } else {
                        let err = format!("Expected command type: {}, got: {}", #command_type, generic_input.command_type);
                        let mut mem = extism_pdk::Memory::from_bytes(&err).unwrap();
                        unsafe {
                            extism_pdk::extism::error_set(mem.offset());
                        }
                    }
                }, 
                Err(e) => {
                    let err = format!("{:?}", e);
                    let mut mem = extism_pdk::Memory::from_bytes(&err).unwrap();
                    unsafe {
                        extism_pdk::extism::error_set(mem.offset());
                    }
                    return -1;
                }
            }   

            0
        }
    }
    .into()
}


fn get_type_string(ty: &syn::Type) -> String{
    match ty {
        syn::Type::Path(p) => {
            let mut segments = p.path.segments.iter().peekable();
            let mut path = String::new();

            while let Some(segment) = segments.next()  {
                let ident = &segment.ident;
                path = format!("{}{}", path, ident);

                if segments.next().is_none() {
                    if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
                        let arg_paths = args.args.iter().map(|arg| {
                            match arg {
                                syn::GenericArgument::Type(ty) => {
                                    get_type_string(ty)
                                },
                                _ => panic!("listener_fn expects a type parameter, not {:?}", arg),
                            }
                        }).join(", ");
                        path = format!("{}<{}>", path, arg_paths);   
                    }
                } else {
                    path = format!("{}::", path);
                }
            }

            path
        },
        syn::Type::Tuple(t) =>{
            if !t.elems.is_empty() {
                panic!("listener_fn expects a path or none, not a tuple");
            }
            "()".to_string()
        },
        _ => panic!("listener_fn expects a path parameter, not {:?}", ty),
    }
}


#[proc_macro]
pub fn json_schema(_input: TokenStream) -> TokenStream {
    use ignore::Walk;
    use std::{fs, path::PathBuf};

    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cwd = std::path::Path::new(&dir);
    let path = cwd.join("src");
    
    let walk: Vec<PathBuf> = Walk::new(path)
        .map(|e| e.unwrap())
        .filter(|e| e.path().extension().map(|e| e == "rs").unwrap_or(false))
        .map(|e| e.into_path())
        .collect();

    let mut structs: Vec<(String, syn::ItemStruct)> = vec![];

    for entry in walk {
        let contents = fs::read_to_string(&entry).expect("Could not read file");
        if !contents.contains("JsonSchema") {
            continue;
        }

        let ast = syn::parse_file(&contents).expect("Could not parse file");
        structs.extend(get_structs(ast));
    }

    let structs = structs.iter().map(|(key, s)| {
        let ident = &s.ident;
        let fields = &s.fields;

        quote::quote! {
            #[derive(serde::Deserialize, schemars::JsonSchema)]
            struct #ident #fields

            json.insert(#key, serde_json::to_value(schemars::schema_for!(#ident)).unwrap());
        }
    });

    TokenStream::from(quote::quote! {
        {
            let mut json = std::collections::HashMap::<&str, serde_json::Value>::new();
            #(#structs)*
            serde_json::to_string(&json).unwrap()
        }
    })
}

#[proc_macro]
pub fn a_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
        fn anwser() -> i32 {
            5
        }
    ))
}

fn get_structs(ast: syn::File) -> Vec<(String, syn::ItemStruct)> {
    ast.items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Struct(s) => {
                let attrs = get_attributes(&s.attrs);
                if attrs.contains(&"JsonSchema".to_string()) {
                    let key = s.ident.to_string();
                    Some((key.clone(), s))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

fn get_attributes(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for attr in attrs {
        let Some(ident) = attr.path().get_ident() else {
            continue;
        };

        if ident == "derive" {
            result.extend(attr.parse_args::<Derive>().unwrap().to_list());
        }
    }
    result
}

struct Derive {
    inner: syn::punctuated::Punctuated<syn::Path, syn::Token![,]>,
}

impl Derive {
    pub(crate) fn to_list(&self) -> Vec<String> {
        self.inner
            .iter()
            .cloned()
            .map(|v| v.segments.last().as_ref().unwrap().ident.to_string())
            .collect()
    }
}

impl Parse for Derive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Derive {
            inner: input.parse_terminated(syn::Path::parse_mod_style, syn::Token![,])?,
        })
    }
}
