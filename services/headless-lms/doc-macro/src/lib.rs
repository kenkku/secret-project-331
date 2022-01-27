use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, ItemFn, Path, PathArguments,
    ReturnType, Type, TypePath,
};

/// Includes the type's JSON example generated by doc-file-generator as a string.
/// Convenience alias for #[cfg_attr(doc, doc = generated_docs!(MyType))]
#[proc_macro_attribute]
pub fn generated_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !cfg!(doc) {
        // skip for non-doc builds
        return item;
    }

    let mut item = syn::parse_macro_input!(item as ItemFn);

    let storage;
    let arg = if attr.is_empty() {
        extract_json_return_type(&item).expect("failed to parse return type")
    } else {
        storage = syn::parse_macro_input!(attr as Type);
        &storage
    };
    let attr: Attribute = syn::parse_quote!(#[cfg(doc = generated_docs!(#arg))]);

    item.attrs.push(attr);

    item.into_token_stream().into()
}

fn extract_json_return_type(item: &ItemFn) -> Result<&Type, &'static str> {
    // should have a path return type
    if let ReturnType::Type(_, b) = &item.sig.output {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = b.as_ref()
        {
            let segment = segments
                .first()
                .expect("return type path shouldn't be empty");
            // return type should have generics (ControllerResult)
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                &segment.arguments
            {
                let arg = args
                    .first()
                    .expect("return type generic list shouldn't be empty");
                // first generic arg should be a path (Json)
                if let GenericArgument::Type(Type::Path(TypePath {
                    path: Path { segments, .. },
                    ..
                })) = arg
                {
                    for seg in segments {
                        if seg.ident == "Json" {
                            // Json should have generics
                            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                args,
                                ..
                            }) = &seg.arguments
                            {
                                if let Some(GenericArgument::Type(t)) = args.first() {
                                    return Ok(t);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err("invalid return type")
}
