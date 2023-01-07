// https://leetcode.com/problems/parallel-courses-iii/description/

use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let len = time.len();
        let mut hm: HashMap<usize, Vec<usize>> = (0..n).map(|idx| (idx as usize, vec![])).collect();
        let mut prereqs = vec![0; len + 1];
        let mut total_time = time.clone();
        let mut result = 0;
        for relation in relations {
            hm.get_mut(&(relation[0] as usize - 1))
                .unwrap()
                .push(relation[1] as usize - 1);
            prereqs[relation[1] as usize - 1] += 1;
        }

        let mut q = VecDeque::new();
        for (course, prereq) in prereqs.iter().enumerate() {
            if *prereq == 0 {
                q.push_back(course as usize);
            }
        }

        while let Some(course) = q.pop_front() {
            let course_total_time = total_time[course];
            result = result.max(course_total_time);
            for &next_course in hm.get(&course).unwrap() {
                let next_total_time = course_total_time + time[next_course];
                total_time[next_course] = total_time[next_course].max(next_total_time);
                prereqs[next_course] -= 1;
                if prereqs[next_course] == 0 {
                    q.push_back(next_course);
                }
            }
        }

        return result;
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
