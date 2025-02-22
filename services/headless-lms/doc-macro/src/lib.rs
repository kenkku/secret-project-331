mod example;
mod generated_doc;

use proc_macro::TokenStream;

/// Includes the type's JSON example generated by doc-file-generator as a string.
/// Convenience alias for #[cfg_attr(doc, doc = generated_docs!(MyType))]
#[proc_macro_attribute]
pub fn generated_doc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    generated_doc::generated_doc_impl(item)
}

#[proc_macro_attribute]
pub fn generated_doc_inner(attr: TokenStream, item: TokenStream) -> TokenStream {
    generated_doc::generated_doc_inner_impl(attr, item)
}

/// Accepts a struct or enum literal and generates an Example impl for the type.
#[proc_macro]
#[track_caller]
pub fn example(input: TokenStream) -> TokenStream {
    example::example_impl(input)
}
