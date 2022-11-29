// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/

pub trait Solution {
    fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    // Sort first by attack, and then by defense in the reverse order.
    //
    // E.g.
    // [1, 10], [2, 8], [2, 6], [3, 7], [4, 8]
    //
    // Once sorted, a character on the right will have attack greater than or
    // equal to the character on the left. We will walk through the characters
    // and compare only defense. If a character on the right has a defense
    // greater than the right, we know the left is a weak character. Because of
    // our sorting strategy, two characters with the same attack cannot be
    // placed such that the right character has a greater defense.
    //
    // We then maintain a stack, and pop any character that has a lower defense,
    // and we increment the counter.
    fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                return b[1].cmp(&a[1]);
            }
            return a[0].cmp(&b[0]);
        });

        let mut result = 0;
        let mut stack: Vec<i32> = Vec::new();
        for prop in properties {
            while !stack.is_empty() && stack.last().unwrap() < &prop[1] {
                stack.pop();
                result += 1;
            }
            stack.push(prop[1]);
        }

        return result;
    }
}

impl Solution for Solution2 {
    // Sort strategy is the same, but we don't need a stack (again!!! _(:з」∠)_)
    // Instead, we go from the right to the left, and maintain a max defense
    // value. Whenever the character on the left side has a lower defense, we
    // increment the result. Otherwise we update max defense.
    fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                return b[1].cmp(&a[1]);
            }
            return a[0].cmp(&b[0]);
        });
        let mut result = 0;
        let mut max_defense = properties[properties.len() - 1][1];
        for idx in (0..properties.len() - 1).rev() {
            let curr_defense = properties[idx][1];
            if curr_defense < max_defense {
                result += 1;
            } else {
                max_defense = curr_defense;
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution1 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                Solution1::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
                0
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution1::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
                1
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution1::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
                1
            );
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                Solution2::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
                0
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution2::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
                1
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution2::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
                1
            );
        }
    }
}
