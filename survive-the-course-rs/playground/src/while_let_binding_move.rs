#[derive(Debug)]
struct SomeStruct {
    val: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deconstruct_move() {
        // Using tuples (even singles) will cause compile error
        let mut maybe_s = Some(SomeStruct { val: 0 });
        if let Some(s) = maybe_s {
            maybe_s = Some(s);
        }
        println!("{:?}", maybe_s);
    }
}
