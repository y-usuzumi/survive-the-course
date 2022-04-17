pub struct Solution;

impl Solution {
    pub fn find_min_array(arr: &mut Vec<i32>, mut k: usize) -> Vec<i32> {
        let mut result = Vec::new();
        while k > 0 && !arr.is_empty() {
            let mut min_idx = usize::MAX;
            let mut min_val = i32::MAX;
            if k >= arr.len() - 1 {
                for (idx, val) in arr.iter().enumerate() {
                    if *val < min_val {
                        min_idx = idx;
                        min_val = *val;
                    }
                }
            } else {
                for idx in 0..=k as usize {
                    let val = arr[idx];
                    if val < min_val {
                        min_idx = idx;
                        min_val = val;
                    }
                }
            }
            result.push(min_val);
            arr.remove(min_idx);
            k -= min_idx;
        }
        result.append(arr);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_min_array(&mut vec![5, 3, 1], 2), vec![1, 5, 3]);
    }

    #[test]
    fn test_2() {
        
        assert_eq!(Solution::find_min_array(&mut vec![8, 9, 11, 2, 1], 3), vec![2, 8, 9, 11, 1]);
    }
}