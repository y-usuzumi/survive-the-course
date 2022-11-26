// https://leetcode.com/problems/text-justification/

pub struct Solution;

impl Solution {
    fn spaces(count: usize) -> String {
        String::from_utf8(vec![b' '; count]).unwrap()
    }
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut curr_words: Vec<&str> = Vec::new();
        let mut curr_width = 0;
        for idx in 0..words.len() {
            if curr_width > 0 && curr_width + words[idx].len() + 1 > max_width {
                if curr_words.len() == 1 {
                    result.push(
                        curr_words[0].to_string() + &Self::spaces(max_width - curr_words[0].len()),
                    );
                } else {
                    let gaps = (max_width
                        - curr_words.iter().map(|w: &&str| w.len()).sum::<usize>())
                        / (curr_words.len() - 1);
                    let extra_spaces = (max_width - curr_width) % (curr_words.len() - 1);
                    let mut curr_string = String::from(curr_words[0]);
                    for idx in 1..=extra_spaces {
                        curr_string.push_str(&Self::spaces(gaps + 1));
                        curr_string.push_str(curr_words[idx]);
                    }
                    for idx in extra_spaces + 1..curr_words.len() {
                        curr_string.push_str(&Self::spaces(gaps));
                        curr_string.push_str(curr_words[idx]);
                    }
                    result.push(curr_string);
                }
                curr_words.clear();
                curr_width = 0;
            }
            curr_words.push(&words[idx]);
            curr_width += words[idx].len();
            if curr_words.len() > 1 {
                curr_width += 1;
            }
        }
        if curr_words.len() > 0 {
            let mut curr_string = String::from(curr_words[0]);
            let mut curr_width = curr_words[0].len();
            for idx in 1..curr_words.len() {
                curr_string.push(' ');
                curr_string.push_str(curr_words[idx]);
                curr_width += curr_words[idx].len() + 1;
            }
            result.push(curr_string + &Self::spaces(max_width - curr_width));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use test_util::strs_into_strings;

    use super::*;

    #[test]
    fn test_1() {
        let words = strs_into_strings(vec![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]);
        assert_eq!(
            Solution::full_justify(words, 16),
            strs_into_strings(vec![
                "This    is    an",
                "example  of text",
                "justification.  "
            ])
        );
    }

    #[test]
    fn test_2() {
        let words = strs_into_strings(vec!["What", "must", "be", "acknowledgment", "shall", "be"]);
        assert_eq!(
            Solution::full_justify(words, 16),
            strs_into_strings(vec![
                "What   must   be",
                "acknowledgment  ",
                "shall be        "
            ])
        );
    }

    #[test]
    fn test_3() {
        let words = strs_into_strings(vec![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ]);
        assert_eq!(
            Solution::full_justify(words, 20),
            strs_into_strings(vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ])
        );
    }
}
