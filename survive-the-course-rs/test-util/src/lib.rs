use std::fmt::Debug;

pub fn strs_into_strings(f: Vec<&str>) -> Vec<String> {
    f.into_iter().map(|s| s.to_string()).collect()
}

pub trait EqIgnoreOrdRec: Ord + Clone + Debug {
    fn are_equal_ignore_ord_rec(&self, other: &Self) -> bool;
}

impl EqIgnoreOrdRec for i32 {
    fn are_equal_ignore_ord_rec(&self, other: &Self) -> bool {
        return self == other;
    }
}

impl EqIgnoreOrdRec for String {
    fn are_equal_ignore_ord_rec(&self, other: &Self) -> bool {
        return self == other;
    }
}

impl<T: EqIgnoreOrdRec> EqIgnoreOrdRec for Vec<T> {
    fn are_equal_ignore_ord_rec(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut sorted_self = self.clone();
        sorted_self.sort();
        let mut sorted_other = other.clone();
        sorted_other.sort();
        for (va, vb) in sorted_self.into_iter().zip(sorted_other) {
            if !va.are_equal_ignore_ord_rec(&vb) {
                return false;
            }
        }
        return true;
    }
}

pub fn assert_eq_ignore_order<T: EqIgnoreOrdRec>(a: T, b: T) {
    assert!(
        &a.are_equal_ignore_ord_rec(&b),
        "{:?} is not equal to (ignore order) {:?}",
        a,
        b
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq_ignore_order(vec![1, 2, 3], vec![2, 3, 1]);
        assert_eq_ignore_order(
            vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]],
            vec![vec![7, 6, 9, 8], vec![1, 3, 2], vec![5, 4]],
        );
        assert_eq_ignore_order(
            vec![
                vec![vec![1, 2], vec![3]],
                vec![vec![4, 5, 6], vec![7, 8, 9, 10]],
            ],
            vec![
                vec![vec![8, 9, 10, 7], vec![5, 6, 4]],
                vec![vec![2, 1], vec![3]],
            ],
        );
    }
}
