// https://leetcode.com/problems/minimum-time-difference/

pub struct Solution;

impl Solution {
    fn parse_time(s: &str) -> (i32, i32) {
        let chars: Vec<char> = s.chars().collect();
        let hours = (chars[0] as i32 - '0' as i32) * 10 + chars[1] as i32 - '0' as i32;
        let minutes = (chars[3] as i32 - '0' as i32) * 10 + chars[4] as i32 - '0' as i32;

        return (hours, minutes);
    }

    fn time_diff((h1, m1): (i32, i32), (h2, m2): (i32, i32)) -> i32 {
        return (h2 - h1) * 60 + m2 - m1;
    }

    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() < 2 {
            return 0;
        }
        let mut time_points: Vec<(i32, i32)> = time_points
            .into_iter()
            .map(|tp| Self::parse_time(&tp))
            .collect();
        time_points.sort();
        let mut min = i32::MAX;
        for idx in 1..time_points.len() {
            min = min.min(Self::time_diff(time_points[idx - 1], time_points[idx]));
        }
        min = min.min(Self::time_diff(
            time_points[time_points.len() - 1],
            (time_points[0].0 + 24, time_points[0].1),
        ));

        return min;
    }
}

#[cfg(test)]
mod tests {
    use test_util::strs_into_strings;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_min_difference(strs_into_strings(vec!["23:59", "00:00"])),
            1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_min_difference(strs_into_strings(vec!["00:00", "23:59", "00:00"])),
            0
        );
    }
}
