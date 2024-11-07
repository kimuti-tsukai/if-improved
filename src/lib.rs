use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::Parse, parse_macro_input, Attribute, Block, Token
};

#[derive(Clone, Debug)]
enum Expr {
    BuiltIn(syn::Expr),
    MyExprIf(MyExprIf),
}

impl Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        <MyExprIf as Parse>::parse(input)
            .map(Expr::MyExprIf)
            .or(<syn::Expr as Parse>::parse(input).map(Expr::BuiltIn))
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let new_tokens = match self {
            Self::BuiltIn(c) => quote! { #c },
            Self::MyExprIf(c) => quote! { #c },
        };

        tokens.append_all(new_tokens);
    }
}

#[derive(Clone, Debug)]
struct MyExprIf {
    attrs: Vec<Attribute>,
    _if_token: Token![if],
    cond: Box<Expr>,
    guard: Option<(Token![if], Box<Expr>)>,
    then_branch: Block,
    else_branch: Option<(Token![else], Box<Expr>)>,
}

impl Parse for MyExprIf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;

        let _if_token: Token![if] = input.parse()?;

        let cond: Box<Expr> = Box::new(Expr::parse(input)?);

        let guard = if input.peek(Token![if]) {
            let _if_token: Token![if] = input.parse()?;

            let expr: Expr = Expr::parse(input)?;

            Some((_if_token, Box::new(expr)))
        } else {
            None
        };

        let then_branch: Block = input.parse()?;

        let else_branch = if input.peek(Token![else]) {
            let else_token: Token![else] = input.parse()?;

            let expr: Expr = input.parse()?;

            Some((else_token, Box::new(expr)))
        } else {
            None
        };

        Ok(MyExprIf {
            attrs,
            _if_token,
            cond,
            guard,
            then_branch,
            else_branch,
        })
    }
}

impl ToTokens for MyExprIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let MyExprIf {
            attrs,
            _if_token: _,
            cond,
            guard,
            then_branch,
            else_branch,
        } = self;

        let new_tokens = match (guard, else_branch) {
            (Some((_, guard_cond)), Some((_, else_branch))) => quote! {
                #(#attrs)*
                if #cond {
                    if #guard_cond #then_branch else #else_branch
                } else #else_branch
            },
            (Some((_, guard_cond)), None) => quote! {
                #(#attrs)*
                if #cond {
                    if #guard_cond #then_branch
                }
            },
            (None, Some((_, else_branch))) => quote! {
                #(#attrs)*
                if #cond then_branch else #else_branch
            },
            (None, None) => quote! {
                #(#attrs)*
                if #cond then_branch
            },
        };

        tokens.append_all(new_tokens);
    }
}

#[proc_macro]
pub fn ifi(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as MyExprIf);

    input.to_token_stream().into()
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn parse_my_if() {
        use syn::parse_quote;

        let tokens: MyExprIf = parse_quote!(
            if let Some(v) = Some(100) if v % 2 == 0 {
                println!("{} is an Odd", v);
            } else {
                println!("not an Odd");
            }
        );

        let _i: syn::ExprIf = syn::parse2(tokens.to_token_stream()).unwrap();
    }
}
