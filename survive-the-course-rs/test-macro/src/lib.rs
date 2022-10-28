use proc_macro::{token_stream, Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn solutions(attrs: TokenStream, input: TokenStream) -> TokenStream {
    unimplemented!("Implement me to simplify test cases for multiple solutions")
}

#[proc_macro_attribute]
pub fn unconfident(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
