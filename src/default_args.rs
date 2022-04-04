use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse, punctuated, Token};

use crate::utils::{capitalize, inner_attributes, outer_attributes};

#[derive(Debug)]
pub struct DefaultFn {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub sig: KeyWordSignature,
    pub block: Box<syn::Block>,
}

impl parse::Parse for DefaultFn {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        let sig: KeyWordSignature = input.parse()?;

        let content: parse::ParseBuffer;
        let brace_token: syn::token::Brace = syn::braced!(content in input);

        let input: parse::ParseStream = &content;
        while input.peek(Token![#]) && input.peek2(Token![!]) {
            attrs.push(input.call(single_parse_inner)?);
        }

        let stmts: Vec<syn::Stmt> = content.call(syn::Block::parse_within)?;

        Ok(Self {
            attrs,
            vis,
            sig,
            block: Box::new(syn::Block { brace_token, stmts }),
        })
    }
}

impl ToTokens for DefaultFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // struct declaration
        let typed_fields = self
            .sig
            .inputs_with_defaults
            .iter()
            .map(|arg| match &*arg.pat {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
                    let arg_name = ident;
                    let arg_type = &arg.ty;
                    quote! { #arg_name: #arg_type }
                }
                _ => {
                    /* handled in parsing */
                    unreachable!()
                }
            });

        let field_with_defaults = self
            .sig
            .inputs_with_defaults
            .iter()
            .map(|arg| match &*arg.pat {
                syn::Pat::Ident(syn::PatIdent {
                    ident: field_name, ..
                }) => {
                    let val = &arg.default_value;
                    quote! { #field_name: #val }
                }
                _ => {
                    /* handled in parsing */
                    unreachable!()
                }
            });

        // the struct's (and it's field's) visibility is taken from function's
        // so that you can use generated struct where you can use the function.
        let vis = &self.vis;

        let struct_name =
            quote::format_ident!("{}Args", capitalize(&mut self.sig.ident.to_string()),);

        let generics_of_default_args = self.sig.inputs_with_defaults.pairs().filter_map(|p| {
            if self.sig.generics.params.pairs().any(|ty_par| {
                *match ty_par.value() {
                    syn::GenericParam::Type(ty) => &ty.ident,
                    syn::GenericParam::Lifetime(lt) => &lt.lifetime.ident,
                    syn::GenericParam::Const(co) => &co.ident,
                } == p.value().ty.to_token_stream().to_string()
            }) {
                Some(&p.value().ty)
            } else {
                None
            }
        });
        let where_clause = &self.sig.generics.where_clause;
        let generics_decl = generics_of_default_args.clone();
        let strukt = quote! {
            #[allow(unused)]
            // #[allow(non_camel_case_types)]
            #vis struct #struct_name <#(#generics_decl),*> #where_clause {
                #vis #(#typed_fields),*
            }
        };
        tokens.extend([strukt]);

        let generics_decl = generics_of_default_args.clone();
        let default_impl = quote! {
            impl <#(#generics_decl),*>::core::default::Default for #struct_name <#(#generics_of_default_args),*> #where_clause {
                fn default() -> Self {
                    Self {
                        #(#field_with_defaults),*
                    }
                }
            }
        };
        tokens.extend([default_impl]);

        // function declaration
        tokens.append_all(outer_attributes(&self.attrs));
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.brace_token.surround(tokens, |tokens| {
            tokens.append_all(inner_attributes(&self.attrs));
            tokens.append_all(&self.block.stmts);
        });
    }
}

fn single_parse_inner(input: parse::ParseStream) -> syn::Result<syn::Attribute> {
    let content: parse::ParseBuffer;
    let bracket_token: syn::token::Bracket = syn::bracketed!(content in input);
    let path: syn::Path = content.call(syn::Path::parse_mod_style)?;
    let tokens = content.parse()?;
    Ok(syn::Attribute {
        pound_token: input.parse()?,
        style: syn::AttrStyle::Inner(input.parse()?),
        bracket_token,
        path,
        tokens,
    })
}

#[derive(Debug)]
pub struct KeyWordSignature {
    pub constness: Option<Token![const]>,
    pub asyncness: Option<Token![async]>,
    pub unsafety: Option<Token![unsafe]>,
    pub abi: Option<syn::Abi>,
    pub fn_token: Token![fn],
    pub ident: proc_macro2::Ident,
    pub generics: syn::Generics,
    pub paren_token: syn::token::Paren,
    pub inputs: punctuated::Punctuated<SimpleFnArg, Token![,]>,
    pub inputs_with_defaults: punctuated::Punctuated<FnArgWithDefault, Token![,]>,
    pub output: syn::ReturnType,
}

impl parse::Parse for KeyWordSignature {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let constness: Option<Token![const]> = input.parse()?;
        let asyncness: Option<Token![async]> = input.parse()?;
        let unsafety: Option<Token![unsafe]> = input.parse()?;
        let abi: Option<syn::Abi> = input.parse()?;
        let fn_token: Token![fn] = input.parse()?;
        let ident: proc_macro2::Ident = input.parse()?;
        let mut generics: syn::Generics = input.parse()?;

        let content: parse::ParseBuffer;
        let paren_token: syn::token::Paren = syn::parenthesized!(content in input);
        let inputs = parse_fn_args(&content)?;
        let inputs_with_defaults = parse_fn_args_with_defaults(&content)?;

        let output: syn::ReturnType = input.parse()?;
        generics.where_clause = input.parse()?;

        Ok(Self {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            inputs_with_defaults,
            output,
        })
    }
}

impl ToTokens for KeyWordSignature {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.constness.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            for input in self.inputs.pairs() {
                match input {
                    punctuated::Pair::Punctuated(input, comma) => {
                        input.to_tokens(tokens);
                        comma.to_tokens(tokens);
                    }
                    punctuated::Pair::End(input) => {
                        input.to_tokens(tokens);
                        <Token![,]>::default().to_tokens(tokens);
                    }
                }
            }
            let struct_name =
                quote::format_ident!("{}Args", capitalize(&mut self.ident.to_string()));
            let fields = self.inputs_with_defaults.iter().map(|arg| match &*arg.pat {
                syn::Pat::Ident(syn::PatIdent {
                    ident: field_name,
                    attrs,
                    by_ref,
                    mutability,
                    subpat: _,
                }) => {
                    quote! { #(#attrs),* #by_ref #mutability #field_name }
                }
                _ => {
                    /* handled in parsing */
                    unreachable!()
                }
            });

            let generics_of_default_args = self.inputs_with_defaults.pairs().filter_map(|p| {
                if self.generics.params.pairs().any(|ty_par| {
                    *match ty_par.value() {
                        syn::GenericParam::Type(ty) => &ty.ident,
                        syn::GenericParam::Lifetime(lt) => &lt.lifetime.ident,
                        syn::GenericParam::Const(co) => &co.ident,
                    } == p.value().ty.to_token_stream().to_string()
                }) {
                    Some(&p.value().ty)
                } else {
                    None
                }
            });
            let arg =
                quote! { #struct_name { #(#fields),* }: #struct_name <#(#generics_of_default_args),*> };
            tokens.extend([arg]);
        });
        self.output.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct SimpleFnArg {
    pub attrs: Vec<syn::Attribute>,
    pub pat: Box<syn::Pat>,
    pub colon_token: Token![:],
    pub ty: Box<syn::Type>,
}

impl parse::Parse for SimpleFnArg {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let pat: Box<syn::Pat> = Box::new(multi_pat(input)?);
        let colon_token: Token![:] = input.parse()?;
        let ty: Box<syn::Type> = Box::new(input.parse()?);

        Ok(Self {
            attrs,
            pat,
            colon_token,
            ty,
        })
    }
}

impl quote::ToTokens for SimpleFnArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(outer_attributes(&self.attrs));
        self.pat.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct FnArgWithDefault {
    pub pat: Box<syn::Pat>,
    pub colon_token: Token![:],
    pub ty: Box<syn::Type>,
    pub eq_token: Token![=],
    pub default_value: Box<syn::Expr>,
}

impl parse::Parse for FnArgWithDefault {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let pat: Box<syn::Pat> = Box::new(multi_pat(input)?);
        // TODO: unused, remove this
        match &*pat {
            syn::Pat::Ident(_) => { /* all good */ }
            _ => return Err(syn::Error::new_spanned(pat, "pattern is not an ident")),
        }
        Ok(Self {
            pat,
            colon_token: input.parse()?,
            ty: Box::new(input.parse()?),
            eq_token: input.parse()?,
            default_value: Box::new(input.parse()?),
        })
    }
}

fn parse_fn_args_with_defaults(
    input: parse::ParseStream,
) -> syn::Result<punctuated::Punctuated<FnArgWithDefault, Token![,]>> {
    let mut args = punctuated::Punctuated::new();

    while !input.is_empty() {
        let arg = input.parse()?;
        args.push_value(arg);

        if input.is_empty() {
            break;
        }

        let comma: Token![,] = input.parse()?;
        args.push_punct(comma);
    }

    Ok(args)
}

fn parse_fn_args(
    input: parse::ParseStream,
) -> syn::Result<punctuated::Punctuated<SimpleFnArg, Token![,]>> {
    let mut args = punctuated::Punctuated::new();

    while !input.is_empty() {
        let attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;

        let ahead: parse::ParseBuffer = input.fork();
        if let Ok(Some(dots)) = ahead.parse::<Option<Token![...]>>() {
            return Err(syn::Error::new_spanned(
                dots,
                "variadics are not supported by `default_args` macro",
            ));
        }

        let ahead: parse::ParseBuffer = input.fork();
        if ahead.parse::<FnArgWithDefault>().is_ok() {
            break;
        }

        let mut arg: SimpleFnArg = input.parse()?;
        arg.attrs = attrs;
        args.push_value(arg);
        if input.is_empty() {
            break;
        }

        let comma: Token![,] = input.parse()?;
        args.push_punct(comma);
    }

    Ok(args)
}

fn multi_pat(input: parse::ParseStream) -> syn::Result<syn::Pat> {
    let mut pat: syn::Pat = input.parse()?;
    if input.peek(Token![|]) && !input.peek(Token![||]) && !input.peek(Token![|=]) {
        let mut cases = punctuated::Punctuated::new();
        cases.push_value(pat);
        while input.peek(Token![|]) && !input.peek(Token![||]) && !input.peek(Token![|=]) {
            let punct = input.parse()?;
            cases.push_punct(punct);
            let pat: syn::Pat = input.parse()?;
            cases.push_value(pat);
        }
        pat = syn::Pat::Or(syn::PatOr {
            attrs: Vec::new(),
            leading_vert: None,
            cases,
        });
    }
    Ok(pat)
}

impl ToTokens for FnArgWithDefault {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pat.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        let _ = self.eq_token;
        // default_value is used in the Arg struct's Default impl.
    }
}
