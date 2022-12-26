mod lifetime_and_variance;

static S1: i32 = 1;
static mut S2: &i32 = &128;

/// The code below does not compile because `assign_ref` must work for all lifetimes.
/// The smallest lifetime would be the call to assign_ref. However, by assigning a
/// reference with such a lifetime to a static mutable reference S2, you are enforcing
/// r to live as long as S2, which has the lifetime of 'static, leading to contradiction
/// ```
/// fn assign_ref<'a>(r: &'a i32) {
///     S2 = r;
/// }
/// ```

fn assign_ref2<'a>(from: &'a i32, to: &mut &'a i32) {
    *to = from;
}

#[derive(Debug)]
struct S<'a> {
    r: &'a i32,
}

fn add_s<'a>(s1: &S<'a>, s2: &S<'a>) {
    let s = S { r: &(s1.r + s2.r) };
    println!("{:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_ref_lifetime() {
        let v1 = 3;
        let s1 = S { r: &v1 };
        {
            let v2 = 4;
            let s2 = S { r: &v2 };
            add_s(&s1, &s2);
        }
    }

    #[test]
    fn test_assign_ref2() {
        // OK
        unsafe {
            assign_ref2(&S1, &mut S2);
        }

        // Bad: v1 does not live long enough.
        // unsafe {
        //     let v1 = 1;
        //     assign_ref2(&v1, &mut S2);
        // }
    }
}
