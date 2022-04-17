use proc_macro::{
    Ident, TokenStream, TokenTree,
    token_stream,
};


#[proc_macro_attribute]
pub fn solutions(attrs: TokenStream, input: TokenStream) -> TokenStream {
    unimplemented!("Implement me to simplify test cases for multiple solutions")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
