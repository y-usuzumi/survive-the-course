// https://leetcode.com/problems/task-scheduler/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub trait Solution {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32;
}

// Min heap keyed by the next available time for a task. If there are multiple
// tasks with the same next availble time, always pick the one with the most
// times remaining to run.
// Check https://leetcode.cn/problems/task-scheduler/solution/ren-wu-diao-du-qi-by-leetcode-solution-ur9w/
// for the proof of why this works.
pub struct Solution1;

impl Solution for Solution1 {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counter = HashMap::new();

        for task in tasks {
            *counter.entry(task).or_insert(0) += 1;
        }

        let mut h = BinaryHeap::new();
        for (k, v) in counter {
            h.push((Reverse(1), v, k));
        }

        let mut result = 0;

        while let Some((Reverse(next), v, k)) = h.pop() {
            if next > result {
                result = next;
            } else {
                result += 1;
            }

            if v > 1 {
                h.push((Reverse(next + n + 1), v - 1, k));
            }
        }

        return result;
    }
}

// Construction
// Check the second approach in https://leetcode.cn/problems/task-scheduler/solution/ren-wu-diao-du-qi-by-leetcode-solution-ur9w/
pub struct Solution2;

impl Solution for Solution2 {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if tasks.is_empty() {
            return 0;
        }

        let mut counter = HashMap::new();

        let task_count = tasks.len() as i32;

        for task in tasks {
            *counter.entry(task).or_insert(0) += 1;
        }

        let max_runs = counter.values().max().unwrap();
        let tasks_with_max_runs = counter.values().filter(|v| *v == max_runs).count() as i32;
        let unfilled_total_runs = (n + 1) * (max_runs - 1) + tasks_with_max_runs;
        return unfilled_total_runs.max(task_count as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
