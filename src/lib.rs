#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/refcell/decolor/main/etc/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/refcell/decolor/main/etc/favicon.ico",
    issue_tracker_base_url = "https://github.com/refcell/decolor/issues/"
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Procedural macro that accepts an async function
/// and turns it into a blocking function by wrapping
/// the internals with a blocking call.
///
/// Under the hood, this works by calling attempting
/// to build a tokio [runtime](tokio::runtime::Runtime)
/// and then calling [`block_on(...)`] on the returned
/// runtime. If the runtime construction fails, for example
/// because the runtime is being nested, then [`decolor`]
/// will fallback to using the current [Handle](tokio::runtime::Handle)
/// and then calling [`block_on()`](tokio::runtime::Handle::block_on())
/// on the returned handle.
///
/// # Example
/// ```
/// use tokio::time::{sleep, Duration};
/// use decolor::decolor;
///
/// #[decolor]
/// async fn foo() {
///    sleep(Duration::from_secs(1)).await;
///    println!("Hello, world!");
/// }
///
/// fn main() {
///    foo();
/// }
/// ```
#[proc_macro_attribute]
pub fn decolor(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input as an async function
    let input = parse_macro_input!(input as ItemFn);

    // Extract the function body
    let orig_function_body = input.block;

    // Extract the function name and return type.
    // Then generate a new synchronous function
    // with the same name and return type.
    let fn_name = &input.sig.ident;
    let orig_return_type = &input.sig.output;
    let expanded = quote! {
        fn #fn_name() #orig_return_type {
            match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async move {
                    #orig_function_body
                }),
                Err(_) => tokio::runtime::Handle::current().block_on(async move {
                    #orig_function_body
                }),
            }
        }
    };

    expanded.into()
}
