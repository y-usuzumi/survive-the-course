#[derive(Debug)]
struct S {
    val: i32,
}

impl S {
    fn new(val: i32) -> Self {
        Self { val }
    }
}

fn use_ref(s: &[S]) {
    // s[1] is a syntactic sugar for *s.index(1).
    // Without an additional reference, this will
    // cause a "cannot move out of slice" error.
    let elem = &s[1];
    println!("{:?}", elem);
}

fn use_string_ref(s: &[String]) {
    for elem in s {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_refeence_elem_type() {
        let a: Vec<S> = vec![1, 2, 3].into_iter().map(S::new).collect();
        use_ref(&a);
        use_ref(&a);
    }
}
