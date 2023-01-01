// https://leetcode.com/problems/cycle-length-queries-in-a-tree/

pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut results = Vec::with_capacity(queries.len());
        for q in queries {
            let mut a = q[0];
            let mut b = q[1];
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            let mut result = 0;

            // Move the deeper node up to until at the same level with the other
            // node.
            while Self::base(b) > Self::base(a) {
                result += 1;
                b /= 2;
            }

            // Then move upwards together until they reach the same common
            // ancestor.
            while a != b {
                a /= 2;
                b /= 2;
                result += 2;
            }

            results.push(result + 1);
        }

        results
    }

    fn base(n: i32) -> i32 {
        return 2i32.pow((n as f64).log2() as u32);
    }
}

impl Solution2 {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut results = Vec::with_capacity(queries.len());
        for q in queries {
            let mut a = q[0];
            let mut b = q[1];

            let mut result = 0;
            // If a and b are at the same depth, one will move upwards
            // Else the deeper node will move upwards
            // This will make sure the two pointers will meet at some point
            while a != b {
                if a > b {
                    a /= 2;
                } else {
                    b /= 2;
                }
                result += 1;
            }

            results.push(result + 1);
        }

        results
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
