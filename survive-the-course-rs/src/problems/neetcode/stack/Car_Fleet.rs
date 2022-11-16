// https://leetcode.com/problems/car-fleet/

pub trait Solution {
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32;
}
pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    #[inline]
    fn compute_duration(target: i32, pos: i32, speed: i32) -> f64 {
        (target as f64 - pos as f64) / speed as f64
    }
}

impl Solution for Solution1 {
    // This solution is based on stack
    // 1. Calculate durations from the position and speed of each car.
    // 2. Sort positions. Also sort durations by their corresponding positions.
    // 3. Iterate through positions and durations. For each new duration, pop
    //    anything in the stack that has a smaller duration (runs faster). Then
    //    push the new duration into the stack.
    // 4. Return the size of the stack. Each element represents a fleet.
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 0 {
            return 0;
        }
        let durations: Vec<f64> = speed
            .into_iter()
            .enumerate()
            .map(|(idx, spd)| Self::compute_duration(target, position[idx], spd))
            .collect();
        let mut pos_dur_pairs: Vec<(i32, f64)> = position.into_iter().zip(durations).collect();
        pos_dur_pairs.sort_by(|a, b| a.0.cmp(&b.0));

        let mut stack: Vec<f64> = Vec::new();

        for (_, dur) in pos_dur_pairs {
            while !stack.is_empty() && stack.last().unwrap() <= &dur {
                stack.pop();
            }
            stack.push(dur);
        }

        return stack.len() as i32;
    }
}

impl Solution for Solution2 {
    // This is the fastest submitted solution in leetcode so far. The idea is
    // instead of iterating forward, by iterating in the reverse order, we can
    // get rid of the stack because if a car behind me reaches the target later
    // than me, it definitely forms a new fleet. The same goes with all cars
    // behind it.
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        let mut pos_speed = vec![(0, 0); n];

        for i in 0..n {
            pos_speed[i] = (position[i], speed[i]);
        }

        pos_speed.sort_unstable_by(|a, b| b.cmp(&a));

        // println!("{:?}", pos_speed);

        let mut result = 0;
        let mut current = 0f64;

        for i in 0..n {
            let (pos, speed) = pos_speed[i];
            let time = (target - pos) as f64 / speed as f64;

            if time > current {
                result += 1;
                current = time;
            }
        }
        return result as i32;
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
                Solution1::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
                3
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution1::car_fleet(10, vec![3], vec![3]), 1);
        }

        #[test]
        fn test_3() {
            assert_eq!(Solution1::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                Solution2::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
                3
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution2::car_fleet(10, vec![3], vec![3]), 1);
        }

        #[test]
        fn test_3() {
            assert_eq!(Solution2::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
        }
    }
}
