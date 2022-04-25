pub mod alg;
pub mod ds;
pub mod problems;

#[cfg(test)]
pub mod test {
    struct X {
        pub val: i32
    }

    #[test]
    fn deref_test() {
        let x = X { val: 3 };

        fn func(x: &mut X) {
            func2(*x);
        }

        fn func2(x: X) {

        }
    }
}