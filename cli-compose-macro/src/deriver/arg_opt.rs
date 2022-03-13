use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use crate::doc::extract_doc;

static UNSUPPORTED_SHAPE: &str =
    "#[derive(ArgOpt)] can only be applied to structs with single unnamed field or enums";

#[derive(FromAttributes)]
struct ArgOpt {
    long: Option<syn::LitStr>,

    short: Option<syn::LitChar>,

    #[allow(dead_code)]
    short_only: Option<()>,

    #[allow(dead_code)]
    use_default: Option<()>,
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let attr = ArgOpt::try_from_attributes(&input.attrs)?;

    match &input.data {
        syn::Data::Enum(_) => {
            let enum_name = &input.ident;

            let long = attr
                .as_ref()
                .and_then(|arg_opt| arg_opt.long.clone())
                .map_or_else(
                    || input.ident.to_string().to_case(Case::Kebab),
                    |lit_str| lit_str.value(),
                );

            let flag = match attr.and_then(|arg_opt| arg_opt.short) {
                Some(short) => {
                    quote! { cli_compose::schema::Flag::BothLongAndShort(#long.to_owned(), #short) }
                }
                None => quote! { cli_compose::schema::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(&input.attrs);

            Ok(quote! {
                impl cli_compose::schema::AsArgOpt for #enum_name {
                    fn flag() -> cli_compose::schema::Flag {
                        #flag
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        <#enum_name as std::str::FromStr>::from_str(s).ok()
                    }
                }
            })
        }

        syn::Data::Struct(syn::DataStruct {
            struct_token,
            fields,
            ..
        }) => {
            let field = match fields.iter().collect::<Vec<_>>()[..] {
                [field] => field,
                _ => return Err(syn::Error::new_spanned(struct_token, UNSUPPORTED_SHAPE)),
            };

            let struct_name = &input.ident;
            let long = attr
                .as_ref()
                .and_then(|arg_opt| arg_opt.long.clone())
                .map_or_else(
                    || struct_name.to_string().to_case(Case::Kebab),
                    |lit_str| lit_str.value(),
                );

            let flag = match attr.and_then(|arg_opt| arg_opt.short) {
                Some(short) => {
                    quote! { cli_compose::schema::Flag::BothLongAndShort(#long.to_owned(), #short) }
                }
                None => quote! { cli_compose::schema::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(&input.attrs);

            let ty = field.ty.clone();

            Ok(quote! {
                impl cli_compose::schema::AsArgOpt for #struct_name {
                    fn flag() -> cli_compose::schema::Flag {
                        #flag
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        let val = <#ty as std::str::FromStr>::from_str(s).ok()?;
                        Some(#struct_name(val))
                    }
                }
            })
        }

        syn::Data::Union(data_union) => Err(syn::Error::new_spanned(
            data_union.union_token,
            UNSUPPORTED_SHAPE,
        )),
    }
}
