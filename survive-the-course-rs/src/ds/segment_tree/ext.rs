use super::SegmentTree;

pub fn sum_segment_tree_from_arr(v: &[i32]) -> SegmentTree<i32, i32> {
    SegmentTree::from_arr(v, Box::new(|a| *a), Box::new(|| 0), Box::new(|a, b| a + b))
}

pub fn min_segment_tree_from_arr(v: &[i32]) -> SegmentTree<i32, i32> {
    SegmentTree::from_arr(
        v,
        Box::new(|a| *a),
        Box::new(|| i32::MAX),
        Box::new(|a, b| *a.min(b)),
    )
}
