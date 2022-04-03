use quote::{quote, TokenStreamExt};
use syn::{parse, punctuated};

use crate::utils::{capitalize, outer_attributes};

pub struct KeywordFn {
    pub attrs: Vec<syn::Attribute>,
    pub func: Box<syn::ExprPath>,
    pub paren_token: syn::token::Paren,
    pub args: KeywordFnArgs,
}

impl parse::Parse for KeywordFn {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let func: Box<syn::ExprPath> = input.parse()?;
        let content: parse::ParseBuffer;
        let paren_token: syn::token::Paren = syn::parenthesized!(content in input);
        Ok(Self {
            attrs,
            func,
            paren_token,
            args: content.parse()?,
        })
    }
}

impl quote::ToTokens for KeywordFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(outer_attributes(&self.attrs));
        let mut func_args_path = self.func.path.segments.clone();
        // At least one element in a path.
        let pair = func_args_path.pop().unwrap();
        let func_name = &pair.value().ident;
        func_args_path.push_value(syn::PathSegment {
            ident: quote::format_ident!("{}Args", capitalize(&mut func_name.to_string())),
            arguments: syn::PathArguments::None,
        });
        self.func.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            let positional = self.args.positional_args.pairs().map(|p| *p.value());
            let default_args = self.args.keyword_args.pairs().map(|p| {
                let KeywordArg {
                    pat: field_name,
                    value,
                    ..
                } = p.value();
                quote! { #field_name: #value }
            });
            let arguments = quote! {
                #(#positional,)*
                #[allow(clippy::needless_update)]
                #func_args_path {
                    #(#default_args,)*
                    ..#func_args_path::default()
                }
            };
            tokens.extend([arguments]);
        });
    }
}

pub struct KeywordFnArgs {
    pub positional_args: punctuated::Punctuated<syn::Expr, syn::Token![,]>,
    pub keyword_args: punctuated::Punctuated<KeywordArg, syn::Token![,]>,
}

impl parse::Parse for KeywordFnArgs {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut positional_args = punctuated::Punctuated::new();
        let mut keyword_args = punctuated::Punctuated::new();

        loop {
            if input.is_empty() {
                break;
            }

            if input.peek2(syn::Token![=]) {
                break;
            }

            let arg: syn::Expr = input.parse()?;
            positional_args.push_value(arg);

            if input.is_empty() {
                break;
            }

            let comma: syn::Token![,] = input.parse()?;
            positional_args.push_punct(comma);
        }

        while !input.is_empty() {
            let kw_arg: KeywordArg = input.parse()?;
            keyword_args.push_value(kw_arg);

            if input.is_empty() {
                break;
            }

            let comma: syn::Token![,] = input.parse()?;
            keyword_args.push_punct(comma);
        }

        Ok(Self {
            positional_args,
            keyword_args,
        })
    }
}

pub struct KeywordArg {
    pub pat: Box<syn::Pat>,
    pub eq_token: syn::Token![=],
    pub value: syn::Expr,
}

impl parse::Parse for KeywordArg {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl quote::ToTokens for KeywordArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pat.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}
