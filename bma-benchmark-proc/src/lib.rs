//!  Procedure macros for <https://crates.io/crates/bma-benchmark>
use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use std::panic::panic_any;

const ERR_INVALID_OPTIONS: &str = "Invalid options";

#[proc_macro_attribute]
/// Wraps functions for a staged benchmark
///
/// Attribute options:
///
/// * **i** number of iterations, required
/// * **name** custom stage name (the default is function name)
///
/// If a function name starts with *test_* or *benchmark_*, the prefix is automatically stripped.
///
/// Example:
///
/// ```rust
/// #[benchmark_stage(i=1_000)]
/// fn test1() {
///     // do something
/// }
/// ```
///
/// ```rust
/// #[benchmark_stage(i=1_000,name=stage1)]
/// fn test1() {
///     // do something
/// }
/// ```
///
/// # Panics
///
/// Will panic on invalid options
pub fn benchmark_stage(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).expect("Invalid input");
    let mut args_iter = args.into_iter();
    let mut opt_i: Option<u32> = None;
    let mut opt_name: Option<String> = None;
    macro_rules! parse_opt {
        ($c: block) => {{
            let v = args_iter.next().expect(ERR_INVALID_OPTIONS);
            if let TokenTree::Punct(c) = v {
                if c.as_char() == '=' {
                    $c
                } else {
                    panic_any(ERR_INVALID_OPTIONS);
                }
            } else {
                panic_any(ERR_INVALID_OPTIONS);
            }
        }};
    }
    while let Some(v) = args_iter.next() {
        if let TokenTree::Ident(i) = v {
            let s = i.to_string();
            match s.as_str() {
                "i" => parse_opt!({
                    if let TokenTree::Literal(v) =
                        args_iter.next().expect("Option value not specified")
                    {
                        opt_i = Some(
                            v.to_string()
                                .replace('_', "")
                                .parse()
                                .expect("Invalid integer"),
                        );
                    } else {
                        panic!("Invalid value for \"i\"");
                    }
                }),
                "name" => parse_opt!({
                    match args_iter.next().unwrap() {
                        TokenTree::Literal(v) => opt_name = Some(v.to_string()),
                        TokenTree::Ident(v) => opt_name = Some(v.to_string()),
                        _ => panic!("Invalid value for \"name\""),
                    }
                }),
                _ => panic!("Invalid parameter: {}", s),
            }
        }
    }
    let iterations = opt_i.expect("Iterations not specified");
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn"),
    };
    let mut name = opt_name.unwrap_or_else(|| {
        let n = fn_item.sig.ident.to_string();
        if n.starts_with("test_") {
            n.strip_prefix("test_").unwrap().to_owned()
        } else if n.starts_with("benchmark_") {
            n.strip_prefix("benchmark_").unwrap().to_owned()
        } else {
            n
        }
    });
    if name.starts_with('"') && name.ends_with('"') {
        name = name[1..name.len() - 1].to_owned();
    }
    let fn_block = &fn_item.block;
    fn_item.block.stmts = vec![syn::parse(
        quote!(bma_benchmark::staged_benchmark!(#name, #iterations, #fn_block);).into(),
    )
    .unwrap()];
    item.into_token_stream().into()
}
