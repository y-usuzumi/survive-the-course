pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }

        let (mut idxl, mut idxr) = (0, s.len() - 1);
        while idxl < idxr {
            let temp = s[idxl];
            s[idxl] = s[idxr];
            s[idxr] = temp;
            idxl += 1;
            idxr -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut v);
        assert_eq!(v.into_iter().collect::<String>(), "olleh");
    }
}