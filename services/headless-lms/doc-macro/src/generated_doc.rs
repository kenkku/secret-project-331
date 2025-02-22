use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, ItemFn, Path, PathArguments,
    ReturnType, Type, TypePath,
};

pub fn generated_doc_impl(item: TokenStream) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.extend(
        "#[cfg_attr(doc, generated_doc_inner)]"
            .parse::<TokenStream>()
            .unwrap(),
    );
    stream.extend(item);
    stream
}

pub fn generated_doc_inner_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item as ItemFn);

    let storage;
    let arg = if attr.is_empty() {
        extract_json_return_type(&item).expect("failed to parse return type")
    } else {
        storage = syn::parse_macro_input!(attr as Type);
        &storage
    };
    let attr: Attribute = syn::parse_quote!(#[doc = generated_docs!(#arg)]);

    item.attrs.push(attr);

    item.into_token_stream().into()
}

fn extract_json_return_type(item: &ItemFn) -> Result<&Type, String> {
    // should have a path return type
    if let ReturnType::Type(_, ty) = &item.sig.output {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = ty.as_ref()
        {
            let segment = segments
                .last()
                .expect("return type path shouldn't be empty");

            let mut inner_segments = segments;
            // extract inner segments from non-Json types, use as is otherwise
            if segment.ident != "Json" {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    args, ..
                }) = &segment.arguments
                {
                    // extract inner generic (e.g. T from some::path::Result<T, E>)
                    let arg = args
                        .first()
                        .expect("return type generic list shouldn't be empty");
                    if let GenericArgument::Type(Type::Path(TypePath {
                        path: Path { segments, .. },
                        ..
                    })) = arg
                    {
                        inner_segments = segments;
                    }
                }
            };

            // extract generics e.g. T from Json<T>
            if let Some(json) = inner_segments.last() {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    args, ..
                }) = &json.arguments
                {
                    if let Some(GenericArgument::Type(t)) = args.first() {
                        return Ok(t);
                    }
                }
            }
        }
    }
    Err(format!(
        "return type was expected to be `Json<_>` or `SomeGenericType<Json<_>>`, but it was `{}`",
        item.sig.output.to_token_stream()
    ))
}
