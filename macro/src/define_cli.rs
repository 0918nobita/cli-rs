mod cli_def;
mod cli_defs;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::{cli_def::CliDef, cli_defs::CliDefs};

pub fn define_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let defs: CliDefs = syn::parse2(input)?;

    let contents = defs.into_iter()
        .map(|CliDef { cli_ty, res_ty, members }| {
            let dump_members = members.iter().map(|member| {
                let member_name = member.into_token_stream().to_string();
                quote!{
                    println!(
                        "{: >6} │ {}",
                        format!("{:?}", <#member as cli_compose::schema::AsMember<_>>::kind()),
                        #member_name,
                    );
                }
            }).collect::<TokenStream>();

            quote! {
                #[allow(dead_code)]
                struct #res_ty {
                    input: String,
                    output: Option<String>,
                    stdin: Option<playground_opts::StdinOpt>,
                    stdout: Option<playground_opts::StdoutOpt>,
                    verbose: Option<playground_opts::Verbose>,
                }

                #[allow(dead_code)]
                impl cli_compose::runtime::AsCli<#res_ty> for #cli_ty {
                    fn parse(args: impl Iterator<Item = String>) -> #res_ty {
                        let tokens = cli_compose::runtime::parse_into_tokens(args).collect::<Vec<_>>();
                        println!("tokens: {:?}", tokens);
                        println!("───────┬───────────────────────────────");
                        #dump_members
                        todo!()
                    }
                }
            }
        })
        .collect::<TokenStream>();

    let contents = crate::pretty_print::pretty_print_rust_code(contents).unwrap();

    Ok(quote! {
        let out_dir = std::env::var("OUT_DIR").expect("$OUT_DIR is not set");

        let mut dest = std::path::PathBuf::from(&out_dir).join("cli_compose");

        std::fs::create_dir_all(&dest).expect("Failed to create cli_compose directory");

        dest.push("cli.rs");

        std::fs::write(&dest, #contents).unwrap_or_else(|err| {
            eprintln!("{}", err);
            panic!("Failed to write source file ({:?})", &dest);
        });
    })
}
