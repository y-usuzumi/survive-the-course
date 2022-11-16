// https://leetcode.cn/problems/encode-and-decode-strings/

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// This solution aims at producing human-readable encoded strings.
// A faster alternative is to encode the lengths of the original strings.
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut chars = Vec::new();
        for s in strs {
            for ch in s.chars() {
                match ch {
                    '\0' => {
                        chars.push('\\');
                        chars.push('0');
                    }
                    ';' => {
                        chars.push('\\');
                        chars.push(';');
                    }
                    '\\' => {
                        chars.push('\\');
                        chars.push('\\');
                    }
                    _ => {
                        chars.push(ch);
                    }
                }
            }
            chars.push(';');
        }

        chars.into_iter().collect()
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut escape = false;
        let mut chars = Vec::new();
        for ch in s.chars() {
            match ch {
                '\\' => {
                    if escape {
                        chars.push('\\');
                    }
                    escape = !escape;
                }
                ';' => {
                    if escape {
                        chars.push(';');
                        escape = false;
                    } else {
                        result.push(chars.into_iter().collect());
                        chars = Vec::new();
                    }
                }
                '0' => {
                    if escape {
                        chars.push('\0');
                        escape = false;
                    } else {
                        chars.push('0');
                    }
                }
                _ => {
                    escape = false;
                    chars.push(ch);
                }
            }
        }
        if !chars.is_empty() {
            result.push(chars.into_iter().collect());
        }
        return result;
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_1() {
        let strs = vec!["hello;world", "foo;bar\0"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let codec = Codec::new();
        assert_eq!(
            codec.encode(strs),
            "hello\\;world;foo\\;bar\\0;".to_string()
        );
    }

    #[test]
    fn test_decode_1() {
        let strs: Vec<String> = vec!["hello;world", "foo;bar\0"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let codec = Codec::new();
        assert_eq!(codec.decode(codec.encode(strs.clone())), strs);
    }
}
