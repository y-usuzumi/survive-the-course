fn foo() -> String {
    "Foo2".into()
}

macro_rules! hygiene_test1 {
    () => {
        // Without using the $crate metavariable, the items must be in the scope
        // of the invocation
        foo()
    };
}

macro_rules! hygiene_test2 {
    () => {
        $crate::hygiene::foo()
    };
}

fn bar1() {
    hygiene_test1!();
}

macro_rules! bar2 {
    () => {
        $crate::hygiene_test1!();
    };
}

#[cfg(test)]
mod tests {
    fn foo() -> String {
        "Foo1".into()
    }

    #[test]
    fn test_hygiene_test1() {
        assert_eq!(hygiene_test1!(), "Foo1".to_string());
    }

    #[test]
    fn test_hygiene_test2() {
        assert_eq!(hygiene_test2!(), "Foo2".to_string());
    }
}
