use itertools::Itertools;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

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

    quote! {
        #[no_mangle]
        pub #constness #unsafety extern "C" fn #name() -> i32 {            
            #constness #unsafety fn inner #generics(#input_name: #input_ty) #output #block
            let contour_rust_pdk::extism_pdk::Json(json): contour_rust_pdk::contour_rust_pdk::extism_pdk::Json<serde_json::Value> = contour_rust_pdk::extism_pdk::unwrap!(contour_rust_pdk::extism_pdk::input());
            let input: contour_rust_pdk::io::HandlerInput::<#input_ty> = contour_rust_pdk::extism_pdk::unwrap!(serde_json::from_value(json));

            let output = if input.command_type == #command_type {
                match inner(input.command) {
                    Ok(x) => x,
                    Err(rc) => {
                        let err = format!("{:?}", rc.0);
                        let mut mem = contour_rust_pdk::extism_pdk::Memory::from_bytes(&err).unwrap();
                        unsafe {
                            contour_rust_pdk::extism_pdk::extism::error_set(mem.offset());
                        }

                        return rc.1;
                    }
                };
            };

            contour_rust_pdk::extism_pdk::unwrap!(contour_rust_pdk::extism_pdk::output(&output));
            0
        }
    }
    .into()
}

fn get_type_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(p) => {
            let mut segments = p.path.segments.iter().peekable();
            let mut path = String::new();

            while let Some(segment) = segments.next() {
                let ident = &segment.ident;
                path = format!("{}{}", path, ident);

                if segments.next().is_none() {
                    if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
                        let arg_paths = args
                            .args
                            .iter()
                            .map(|arg| match arg {
                                syn::GenericArgument::Type(ty) => get_type_string(ty),
                                _ => panic!("listener_fn expects a type parameter, not {:?}", arg),
                            })
                            .join(", ");
                        path = format!("{}<{}>", path, arg_paths);
                    }
                } else {
                    path = format!("{}::", path);
                }
            }

            path
        }
        syn::Type::Tuple(t) => {
            if !t.elems.is_empty() {
                panic!("listener_fn expects a path or none, not a tuple");
            }
            "()".to_string()
        }
        _ => panic!("listener_fn expects a path parameter, not {:?}", ty),
    }
}
