struct Solution {
    bad: usize
}

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        let version_usize = version as usize;
        version_usize as usize >= self.bad
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let (mut left, mut right) = (1, n);
        let mut min_bad_version= n;
        while left <= right {
            let mid = right + (left - right) / 2;
            if self.isBadVersion(mid) {
                min_bad_version = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        min_bad_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution{bad: 4}.first_bad_version(5), 4);
        assert_eq!(Solution{bad: 1}.first_bad_version(1), 1);
    }
}