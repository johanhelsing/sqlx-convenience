extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Lit, Result, Token,
};

struct Args {
    table: Lit,
    vars: Vec<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Args> {
        let table = input.parse()?;
        let _: Token![,] = input.parse()?;
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(&input)?;
        Ok(Args {
            table,
            vars: vars.into_iter().collect(),
        })
    }
}

#[proc_macro]
pub fn insert(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as Args);
    let vars = args.vars;
    let table = args.table;
    let dollars: Vec<String> = (1..vars.len() + 1).map(|i| format!("${}", i)).collect();
    let dollars = dollars.join(", ");
    let query_string = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        quote! {#table},
        quote! {#(#vars), *},
        dollars
    );
    let result = quote! {
        sqlx::query!(#query_string, #(#vars), *)
    };
    result.into()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
