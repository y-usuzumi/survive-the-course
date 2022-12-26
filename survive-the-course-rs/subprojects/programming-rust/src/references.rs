#[derive(Debug)]
struct V {
    val: i32,
}

fn add(a: i32, b: i32) -> V {
    V { val: a + b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_eq() {
        let a = 1;
        let b = 1;
        let ra = &a;
        let rb = &b;
        assert_eq!(ra, rb);
    }

    #[test]
    fn test_ptr_eq() {
        let a = 1;
        let b = 1;
        let ra = &a;
        let rb = &b;
        assert!(!std::ptr::eq(ra, rb));
        println!("{:?}", ra as *const i32);
    }
}
