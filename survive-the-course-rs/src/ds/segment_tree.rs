pub mod ext;

pub struct SegmentTree<T, U> {
    vec: Vec<U>,
    len: usize,
    singleton_func: Box<dyn Fn(&T) -> U>,
    default_value_func: Box<dyn Fn() -> U>,
    aggr_func: Box<dyn Fn(&U, &U) -> U>,
}

impl<T, U: Copy> SegmentTree<T, U> {
    pub fn from_arr(
        v: &[T],
        singleton_func: Box<dyn Fn(&T) -> U>,
        default_value_func: Box<dyn Fn() -> U>,
        aggr_func: Box<dyn Fn(&U, &U) -> U>,
    ) -> Self {
        let len = v.len();
        let leaves_len = (2 as u32).pow((len as f64).log2().ceil() as u32) as usize;
        let total_len = 2 * leaves_len - 1;
        let mut vec = Vec::with_capacity(total_len);
        for _ in 0..total_len {
            vec.push(default_value_func());
        }
        let mut st = SegmentTree {
            vec,
            len: v.len(),
            singleton_func,
            default_value_func,
            aggr_func,
        };
        let _ = st.populate(v, 0, 0, len - 1);
        return st;
    }

    fn get_mid(left: usize, right: usize) -> usize {
        return left + ((right - left) / 2);
    }

    fn populate(&mut self, arr: &[T], curr_idx: usize, left: usize, right: usize) -> U {
        if left == right {
            let singleton_value = (self.singleton_func)(&arr[left]);
            self.vec[curr_idx] = singleton_value;
            return singleton_value;
        }
        let mid = Self::get_mid(left, right);
        let left_result = self.populate(arr, curr_idx * 2 + 1, left, mid);
        let right_result = self.populate(arr, curr_idx * 2 + 2, mid + 1, right);
        let aggr_result = (self.aggr_func)(&left_result, &right_result);
        self.vec[curr_idx] = aggr_result;
        return aggr_result;
    }

    fn aggregate_helper(
        &self,
        curr_idx: usize,
        curr_left: usize,
        curr_right: usize,
        left: usize,
        right: usize,
    ) -> U {
        if curr_left >= left && curr_right <= right {
            return self.vec[curr_idx];
        }

        if left > curr_right || right < curr_left {
            return (self.default_value_func)();
        }
        let mid = Self::get_mid(curr_left, curr_right);
        let left_result = self.aggregate_helper(curr_idx * 2 + 1, curr_left, mid, left, right);
        let right_result =
            self.aggregate_helper(curr_idx * 2 + 2, mid + 1, curr_right, left, right);

        let aggr_result = (self.aggr_func)(&left_result, &right_result);
        return aggr_result;
    }

    pub fn aggregate(&self, left: usize, right: usize) -> U {
        self.aggregate_helper(0, 0, self.len - 1, left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sum_segment_tree {
        use crate::ds::segment_tree::ext::sum_segment_tree_from_arr;

        use super::*;

        #[test]
        fn test_underlying_array() {
            let arr = vec![1, 3, 5, 7, 9, 11];
            let st = sum_segment_tree_from_arr(&arr);
            assert_eq!(
                st.vec,
                vec![36, 9, 27, 4, 5, 16, 11, 1, 3, 0, 0, 7, 9, 0, 0]
            );
        }

        #[test]
        fn test_sum_2_4() {
            let arr = vec![1, 3, 5, 7, 9, 11];
            let st = sum_segment_tree_from_arr(&arr);
            assert_eq!(st.aggregate(2, 4), 21);
        }
    }
}
