#[macro_export]
macro_rules! option_vec_element {
    (null) => {
        None
    };
    ($expr:tt) => {
        Some($expr)
    };
}

#[macro_export]
macro_rules! option_vec {
    ($($element:tt),*) => {
        {
            let mut v = Vec::new();
            $(v.push($crate::option_vec_element!($element));)*
            v
        }
    };
    ($($element:tt),+,) => {
        option_vec![$($element),*];
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_vec() {
        let v = option_vec![1, 2, null];
    }
}
