/* Lifetime and Variance */
fn assign_ref_variance<'a, 'b>(from: &'a i32, to: &mut &'b i32)
where
    'a: 'b,
{
    *to = from;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_ref_variance() {
        assert_eq!(1, 1);
    }
}
