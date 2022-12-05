// https://leetcode.com/problems/car-fleet-ii/

pub struct Solution;

impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        if cars.is_empty() {
            return Vec::new();
        }
        let mut result = vec![0.0; cars.len()];
        let mut right_cars_stack = Vec::new();
        for (idx, car) in (cars.iter().enumerate()).rev() {
            let curr_pos = car[0];
            let curr_speed = car[1];

            let mut min_time = f64::MAX;

            if !right_cars_stack.is_empty() {
                let mut prev_car = right_cars_stack[right_cars_stack.len() - 1];
                min_time = Self::get_collide_time((curr_pos, curr_speed), prev_car);
                for idx in (0..right_cars_stack.len() - 1).rev() {
                    let next_car = right_cars_stack[idx];
                    let time = Self::get_collide_time((curr_pos, curr_speed), next_car);
                    if time <= min_time {
                        right_cars_stack.pop();
                        min_time = time;
                    } else {
                        break;
                    }
                }
            }
            right_cars_stack.push((curr_pos, curr_speed));
            result[idx] = if min_time == f64::MAX { -1.0 } else { min_time };
        }

        return result;
    }

    fn get_collide_time(left: (i32, i32), right: (i32, i32)) -> f64 {
        if left.1 <= right.1 {
            // They don't collide
            return f64::MAX;
        }
        return (right.0 - left.0) as f64 / (left.1 - right.1) as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]]),
            vec![1.00000, -1.00000, 3.00000, -1.00000]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::get_collision_times(vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]]),
            vec![2.00000, 1.00000, 1.50000, -1.00000]
        );
    }
}
