extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn ring_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let func_ident = &func.sig.ident;
    let func_attrs = &func.attrs;
    let should_panic = func_attrs
        .iter()
        .find(|&attr| attr.path.is_ident("should_panic"))
        .is_some();
    let test_case_ident = format_ident!("_TEST_CASE{}", func_ident);

    let quote = quote!(

        #func

        #[allow(non_upper_case_globals)]
        #[distributed_slice(TESTCASES)]
        static #test_case_ident: TestCase = TestCase {
            name: concat!(module_path!(),"::",stringify!(#func_ident)),
            func: #func_ident,
            should_panic: #should_panic,
        };
    );

    quote.into()
}