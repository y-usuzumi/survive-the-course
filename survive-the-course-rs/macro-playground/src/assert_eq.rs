// This is a bad implementation because Rust expands the fragments $a and $b as
// is provided by the caller. This means expressions that mutate values may cause
// side effects, for example `bad_assert_eq1!(some_vec.pop(), some_value)`
macro_rules! bad_assert_eq1 {
    ($a:expr, $b:expr) => {
        if $a != $b {
            panic!("Assertion failed: {:?} != {:?}", $a, $b);
        }
    };
}

macro_rules! bad_assert_eq2 {
    ($a:expr, $b:expr) => {
        match ($a, $b) {
            (a, b) => {
                if a != b {
                    panic!("Assertion failed: {:?} != {:?}", a, b);
                }
            }
        }
    };
}

macro_rules! good_assert_eq {
    ($a:expr, $b: expr) => {
        match (&$a, &$b) {
            (a, b) => {
                if a != b {
                    panic!("Assertion failed: {:?} != {:?}", a, b);
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_bad_assert_eq1() {
        #[allow(unused_mut, unused_variables)]
        let mut v = vec![1, 2];
        // calls pop() twice in case assertion fails
        // bad_assert_eq1!(v.pop(), Some(3));
    }

    #[test]
    fn test_bad_assert_eq2() {
        let v = vec!["1".to_string(), "2".to_string()];
        bad_assert_eq2!(v, vec!["1".to_string(), "2".to_string()]);
        // ↓ borrow of moved value
        // assert_eq!(v, vec!["1".to_string()]);
    }

    #[test]
    fn test_good_assert_eq() {
        let mut v = vec!["1".to_string(), "2".to_string()];
        good_assert_eq!(v, vec!["1".to_string(), "2".to_string()]);
        good_assert_eq!(v.pop(), Some("2".to_string()))
    }
}
