use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, ItemStruct, Visibility};

#[proc_macro_attribute]
pub fn go_visibility(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Item);

    match input {
        syn::Item::Struct(item) => handle_struct(item),
        syn::Item::Impl(item) => handle_impl(item),
        _ => TokenStream::from(quote! { #input }),
    }
}

fn handle_struct(item: ItemStruct) -> TokenStream {
    let mut output = item.clone();

    // Handle struct visibility
    if item.ident.to_string().starts_with(char::is_uppercase) {
        output.vis = Visibility::Public(Default::default());
        output.attrs.push(syn::parse_quote! {
            #[allow(non_snake_case)]
        });
    }

    // Handle field visibility
    if let syn::Fields::Named(fields) = &mut output.fields {
        for field in &mut fields.named {
            if field
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                .starts_with(char::is_uppercase)
            {
                field.vis = Visibility::Public(Default::default());
                field.attrs.push(syn::parse_quote! {
                    #[allow(non_snake_case)]
                });
            }
        }
    }

    TokenStream::from(quote! { #output })
}

fn handle_impl(item: ItemImpl) -> TokenStream {
    let mut output = item.clone();

    for item in &mut output.items {
        if let syn::ImplItem::Fn(method) = item {
            if method.sig.ident.to_string().starts_with(char::is_uppercase) {
                method.vis = Visibility::Public(Default::default());
                method.attrs.push(syn::parse_quote! {
                    #[allow(non_snake_case)]
                });
            }
        }
    }

    TokenStream::from(quote! { #output })
}
