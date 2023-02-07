// https://leetcode.com/problems/race-car/

pub struct Solution;

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let size = (target * 2) as usize;
        let mut dfs = vec![vec![false; 33]; size];
        let mut curr_starting_points = vec![(0i32, 1i32)];
        let mut curr_step = 0;
        while !curr_starting_points.is_empty() {
            curr_step += 1;
            let mut next_starting_points = vec![];
            for (point, speed_base) in curr_starting_points {
                dfs[point as usize][(speed_base + 16) as usize] = true;

                let speed = if speed_base > 0 {
                    2i32.pow((speed_base - 1) as u32)
                } else {
                    -(2i32.pow((-speed_base - 1) as u32))
                };

                if point + speed >= 0 && point + speed < size as i32 {
                    let next_point = point + speed;
                    if next_point == target {
                        return curr_step;
                    }
                    let next_speed = if speed_base > 0 {
                        speed_base + 1
                    } else {
                        speed_base - 1
                    };
                    if !dfs[next_point as usize][(next_speed + 16) as usize] {
                        next_starting_points.push((next_point, next_speed));
                    }
                }

                if !dfs[point as usize][((-speed_base).signum() + 16) as usize] {
                    next_starting_points.push((point, (-speed_base).signum()));
                }
            }
            curr_starting_points = next_starting_points;
        }

        panic!("Impossible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::racecar(3), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::racecar(6), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::racecar(4), 5);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::racecar(5), 7);
    }
}
