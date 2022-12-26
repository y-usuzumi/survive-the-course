// #[macro_use]
mod assert_eq;
#[macro_use]
mod hygiene;
#[macro_use]
mod json;

mod avoid_syntax_errors;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_json_macro() {
        let null = json!(null);
        println!("{:?}", null);
    }
}
