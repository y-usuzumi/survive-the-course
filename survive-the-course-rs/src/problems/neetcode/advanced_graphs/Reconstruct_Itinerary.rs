// https://leetcode.com/problems/reconstruct-itinerary/description/

use std::collections::HashMap;

pub struct Solution1;
// Hierholzer's algorithm
pub struct Solution2;

impl Solution1 {
    // TODO: Reorganize the code
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut airport_id_map = HashMap::new();
        let mut idx = 0usize;
        for ticket in &tickets {
            for airport in ticket {
                if !airport_id_map.contains_key(airport) {
                    airport_id_map.insert(airport.clone(), idx);
                    idx += 1;
                }
            }
        }

        let start = *airport_id_map.get("JFK").unwrap();

        let mut id_airport_map: HashMap<usize, String> = airport_id_map
            .iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();

        let mut graph = vec![vec![]; id_airport_map.len()];

        let tickets: Vec<(usize, usize)> = tickets
            .iter()
            .map(|ticket| {
                (
                    *airport_id_map.get(&ticket[0]).unwrap(),
                    *airport_id_map.get(&ticket[1]).unwrap(),
                )
            })
            .collect();
        for (idx, ticket) in tickets.iter().enumerate() {
            graph[ticket.0].push((idx, id_airport_map.get_mut(&ticket.1).unwrap().clone()));
        }

        for flights in &mut graph {
            flights.sort_by_key(|flight| flight.1.clone());
        }

        let mut visited = vec![false; tickets.len()];
        let mut result = vec![0; tickets.len() + 1];

        Self::dfs(
            &mut graph,
            &mut visited,
            0,
            &tickets,
            start,
            start,
            &mut result,
        );
        let result: Vec<String> = result
            .into_iter()
            .map(|airport| id_airport_map.get(&airport).unwrap().clone())
            .collect();
        return result;
    }

    fn dfs(
        g: &Vec<Vec<(usize, String)>>,
        visited: &mut Vec<bool>,
        depth: usize,
        tickets: &Vec<(usize, usize)>,
        start_node: usize,
        node: usize,
        result: &mut Vec<usize>,
    ) -> bool {
        result[depth] = node;
        if depth == tickets.len() {
            return true;
        }
        for (flight_idx, _) in &g[node] {
            if visited[*flight_idx] {
                continue;
            }
            visited[*flight_idx] = true;
            let (_, to) = tickets[*flight_idx];
            if Self::dfs(g, visited, depth + 1, tickets, start_node, to, result) {
                return true;
            }
            visited[*flight_idx] = false;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use test_util::strs_into_strings;

    use super::*;

    #[test]
    fn test_1() {
        let tickets = vec![vec!["JFK", "KUL"], vec!["JFK", "NRT"], vec!["NRT", "JFK"]]
            .into_iter()
            .map(|v| strs_into_strings(v))
            .collect();
        let result = Solution1::find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "NRT", "JFK", "KUL"]);
    }

    #[test]
    fn test_2() {
        let tickets = vec![
            vec!["JFK", "SFO"],
            vec!["JFK", "ATL"],
            vec!["SFO", "ATL"],
            vec!["ATL", "JFK"],
            vec!["ATL", "SFO"],
        ]
        .into_iter()
        .map(|v| strs_into_strings(v))
        .collect();
        let result = Solution1::find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]);
    }

    #[test]
    fn test_3() {
        let tickets = vec![
            vec!["MUC", "LHR"],
            vec!["JFK", "MUC"],
            vec!["SFO", "SJC"],
            vec!["LHR", "SFO"],
        ]
        .into_iter()
        .map(|v| strs_into_strings(v))
        .collect();
        let result = Solution1::find_itinerary(tickets);
        assert_eq!(result, vec!["JFK", "MUC", "LHR", "SFO", "SJC"]);
    }
}
