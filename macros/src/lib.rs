use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn extract_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("extract_fn expects a public function");
    }

    let name = &function.sig.ident;
    let generics = &function.sig.generics;
    let inputs = &mut function.sig.inputs;
    let output = &mut function.sig.output;
    let block = &function.block;

    if name == "main" {
        panic!(
            "extract_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        );
    }

    if !generics.params.is_empty() {
        panic!("extract_fn expects a function with no generics");
    }

    let (input_name, input_ty) = if inputs.len() != 1 {
        panic!("extract_fn expects a function that accepts one parameter");
    } else {
        match inputs.first().unwrap() {
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => match pat.as_ref() {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => (ident, ty),
                _ => panic!("extract_fn expects a function that accepts one parameter"),
            },
            _ => panic!("extract_fn expects a function that accepts one parameter"),
        }
    };

    match output {
        syn::ReturnType::Default => {
            panic!(
                "extract_fn expects a FnResult<()> return value. `Ok(())` may be used if no output is needed"
            )
        }
        syn::ReturnType::Type(_, t) => {
            if let syn::Type::Path(p) = t.as_ref() {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "FnResult" {
                        panic!("extract_fn expects a function that returns FnResult<()>");
                    } else {
                        if let syn::PathArguments::AngleBracketed(args) = &t.arguments {
                            if args.args.len() == 1 {
                                if let Some(syn::GenericArgument::Type(syn::Type::Tuple(tup))) =
                                    args.args.first()
                                {
                                    if !tup.elems.is_empty() {
                                        panic!(
                                            "extract_fn expects a function that returns FnResult<()>"
                                        );
                                    }
                                } else {
                                    panic!(
                                        "extract_fn expects a function that returns FnResult<()>"
                                    );
                                }
                            } else {
                                panic!("extract_fn expects a function that returns FnResult<()>");
                            }
                        }
                    }
                } else {
                    panic!("extract_fn expects a function that returns FnResult<()>");
                }
            }
        }
    };

    token_stream(name, generics, output, block, input_name, input_ty)
}

#[proc_macro_attribute]
pub fn transform_fn(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    if !matches!(function.vis, syn::Visibility::Public(..)) {
        panic!("transform_fn expects a public function");
    }

    let name = &function.sig.ident;
    let generics = &function.sig.generics;
    let inputs = &mut function.sig.inputs;
    let output = &mut function.sig.output;
    let block = &function.block;

    if name == "main" {
        panic!(
            "transform_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        );
    }

    if !generics.params.is_empty() {
        panic!("transform_fn expects a function with no generics");
    }

    let (input_name, input_ty) = if inputs.len() != 1 {
        panic!("transform_fn expects a function that accepts one parameter");
    } else {
        match inputs.first().unwrap() {
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => match pat.as_ref() {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => (ident, ty),
                _ => panic!("transform_fn expects a function that accepts one parameter"),
            },
            _ => panic!("transform_fn expects a function that accepts one parameter"),
        }
    };

    match output {
        syn::ReturnType::Default => {
            panic!("transform_fn expects a FnResult<Change> return value")
        }
        syn::ReturnType::Type(_, t) => {
            if let syn::Type::Path(p) = t.as_ref() {
                if let Some(t) = p.path.segments.last() {
                    if t.ident != "FnResult" {
                        panic!("transform_fn expects a function that returns FnResult<Change>");
                    } else {
                        if let syn::PathArguments::AngleBracketed(args) = &t.arguments {
                            if args.args.len() == 1 {
                                if let Some(syn::GenericArgument::Type(typ)) = args.args.first() {
                                    let ty_string = quote!(#typ).to_string().replace(' ', "");
                                    if !ty_string.contains("Change<") {
                                        panic!(
                                            "transform_fn expects a function that returns FnResult<Change>"
                                        );
                                    }
                                } else {
                                    panic!(
                                        "transform_fn expects a function that returns FnResult<Change>"
                                    );
                                }
                            } else {
                                panic!(
                                    "transform_fn expects a function that returns FnResult<Change>"
                                );
                            }
                        }
                    }
                } else {
                    panic!("transform_fn expects a function that returns FnResult<Change>>");
                }
            }
        }
    };

    token_stream(name, generics, output, block, input_name, input_ty)
}

fn token_stream(
    name: &syn::Ident,
    generics: &syn::Generics,
    output: &mut syn::ReturnType,
    block: &Box<syn::Block>,
    input_name: &syn::Ident,
    input_ty: &Box<syn::Type>,
) -> proc_macro::TokenStream {
    quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #name() -> i32 {
            fn inner #generics(#input_name: #input_ty) #output {
                #block
            }

            let contour_rust_pdk::extism_pdk::Json(input): contour_rust_pdk::extism_pdk::Json<
                contour_rust_pdk::io::HandlerInput::<#input_ty>
            > = contour_rust_pdk::extism_pdk::unwrap!(contour_rust_pdk::extism_pdk::input());

            let output = match inner(input.command) {
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

            contour_rust_pdk::extism_pdk::unwrap!(contour_rust_pdk::extism_pdk::output(&output));
            0
        }
    }
    .into()
}
