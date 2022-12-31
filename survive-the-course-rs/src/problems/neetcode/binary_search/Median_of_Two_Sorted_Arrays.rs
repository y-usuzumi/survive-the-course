// https://leetcode.com/problems/median-of-two-sorted-arrays/

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            // I could do (nums1, nums2) = (nums2, nums1), but
            // the rustc in leetcode is too old and does not
            // support destructuring assignment.
            std::mem::swap(&mut nums1, &mut nums2);
        }

        // Case 1:
        // nums1 = []
        // nums2 = [...]
        if nums1.is_empty() {
            // WHY THE FCK DID I PUT "|" EARLIER???
            return if nums2.len() & 1 == 0 {
                (nums2[nums2.len() / 2] as f64 + nums2[nums2.len() / 2 + 1] as f64) / 2.0
            } else {
                nums2[nums2.len() / 2] as f64
            };
        }

        // I need to get used to using usize which is never negative.
        // Binary search with usize will be in the form of [left, right)
        let total_size = nums1.len() + nums2.len();
        let half_size = total_size / 2;
        let mut left1 = 0;
        let mut right1 = nums1.len();
        while left1 <= right1 {
            // Left half does not include the element at the mid index
            // In this setup, if total size is odd, the median will be
            // included in the right half. For example:
            // nums1 = [1, 2]; nums2 = [3, 4, 5]
            // After binary search, mid1 = 2, mid2 = 0;
            // num11 = 2, num12 = INF
            // num21 = -INF, num22 = 3
            // Median = min(INF, 3) = 3
            let mid1 = (left1 + right1 + 1) / 2;
            let mid2 = half_size - mid1;
            let num11 = if mid1 == 0 {
                f64::MIN
            } else {
                nums1[mid1 - 1] as f64
            };
            let num12 = if mid1 < nums1.len() {
                nums1[mid1] as f64
            } else {
                f64::MAX
            };
            let num21 = if mid2 == 0 {
                f64::MIN
            } else {
                nums2[mid2 - 1] as f64
            };
            let num22 = if mid2 < nums2.len() {
                nums2[mid2] as f64
            } else {
                f64::MAX
            };
            if num11 > num22 {
                right1 = mid1 - 1;
            } else if num21 > num12 {
                left1 = mid1 + 1;
            } else if total_size & 1 == 0 {
                return (num11.max(num21) + num12.min(num22)) / 2.0;
            } else {
                return num12.min(num22);
            }
        }
        panic!("IMPOSSIBLE!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
    }
}
