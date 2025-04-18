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

    if name == "main" {
        panic!(
            "listener_fn must not be applied to a `main` function. To fix, rename this to something other than `main`."
        );
    }

    if !generics.params.is_empty() {
        panic!("listener_fn expects a function with no generics");
    }

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

    quote! {
        #[no_mangle]
        pub #constness #unsafety extern "C" fn #name() -> i32 {
            #constness #unsafety fn inner #generics(#input_name: #input_ty) #output {
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
