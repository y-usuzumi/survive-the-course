struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = Vec::new();
        let mut word_chars = Vec::with_capacity(5);
        for ch in s.chars() {
            if ch == ' ' {
                if !word_chars.is_empty() {
                    Self::reverse(&mut word_chars);
                    words.push(word_chars);
                    word_chars = Vec::with_capacity(5);
                }
                words.push(vec![' ']);
            } else {
                word_chars.push(ch);
            }
        }
        if !word_chars.is_empty() {
            Self::reverse(&mut word_chars);
            words.push(word_chars);
        }
        words.concat().into_iter().collect()
    }

    fn reverse(chars: &mut Vec<char>) {
        if chars.is_empty() {
            return;
        }

        let (mut idxl, mut idxr) = (0, chars.len() - 1);
        while idxl < idxr {
            let temp = chars[idxl];
            chars[idxl] = chars[idxr];
            chars[idxr] = temp;
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
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }
}
