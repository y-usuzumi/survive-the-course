// https://leetcode.com/problems/cheapest-flights-within-k-stops/

use std::collections::HashMap;

pub struct Solution1;
// TODO: Bellman-Ford Algorithm
pub struct Solution2;

impl Solution1 {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut hm = HashMap::new();
        for idx in 0..n {
            hm.insert(idx, vec![]);
        }

        for flight in flights {
            let from = flight[0];
            let to = flight[1];
            let price = flight[2];
            hm.get_mut(&from).unwrap().push((to, price));
        }

        let mut min_cost = vec![i32::MAX; n as usize];
        min_cost[src as usize] = 0;
        let mut q = vec![(src, 0)];
        for _ in 0..=k {
            if q.is_empty() {
                break;
            }
            let mut next_q = vec![];
            for (node, cost) in q {
                if cost < min_cost[node as usize] {
                    min_cost[node as usize] = cost;
                    for child in hm.get(&node).unwrap() {
                        let new_cost = cost + child.1;
                        if new_cost > min_cost[child.0 as usize] {
                            continue;
                        }
                        next_q.push((child.0, new_cost));
                    }
                }
            }
            q = next_q;
        }

        return if min_cost[dst as usize] == i32::MAX {
            -1
        } else {
            min_cost[dst as usize]
        };
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
