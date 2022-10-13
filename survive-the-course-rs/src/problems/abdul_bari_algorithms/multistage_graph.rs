/// MultiStage Graph dynamic programming problem
/// https://www.youtube.com/watch?v=9iE9Mj4m8jk&list=PLDN4rrl48XKpZkf03iYFl-O29szjTrs_O&index=47
use std::collections::{HashMap, HashSet};

pub struct Graph {
    nodes: HashMap<i32, Vec<(i32, i32)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add(&mut self, edge: &[i32; 2], weight: i32) -> &mut Self {
        self.nodes
            .entry(edge[0])
            .or_insert(Vec::new())
            .push((edge[1], weight));

        self.nodes.entry(edge[1]).or_insert(Vec::new());

        self
    }

    fn root_nodes(&self) -> HashSet<i32> {
        let mut all_nodes = self.nodes.keys().map(|k| *k).collect::<HashSet<i32>>();
        for (_, edges) in &self.nodes {
            for (target, _) in edges {
                all_nodes.remove(target);
            }
        }
        all_nodes
    }

    fn leaf_nodes(&self) -> HashSet<i32> {
        self.nodes
            .iter()
            .filter_map(|(k, v)| v.is_empty().then(|| *k))
            .collect::<HashSet<i32>>()
    }
}

pub struct Solution;

impl Solution {
    pub fn shortest_path(graph: Graph) -> (Vec<i32>, i32) {
        let maybe_source = graph.root_nodes().into_iter().next();
        match maybe_source {
            None => (Vec::new(), -1),
            Some(source) => {
                // HashMap<target, (source, length)>
                let mut table: HashMap<i32, (i32, i32)> = HashMap::new();
                let mut next_nodes = HashSet::new();
                next_nodes.insert(source);
                let mut next_next_nodes = HashSet::new();
                while !next_nodes.is_empty() {
                    for curr_node in next_nodes {
                        let (prev_node, curr_length) =
                            table.get(&curr_node).unwrap_or(&(-1, 0)).clone();
                        for (next_node, weight) in graph.nodes.get(&curr_node).unwrap() {
                            let source_and_length = table
                                .entry(*next_node)
                                .or_insert((curr_node, curr_length + *weight));
                            if curr_length + weight < source_and_length.1 {
                                source_and_length.0 = curr_node;
                                source_and_length.1 = curr_length + weight
                            }
                            next_next_nodes.insert(*next_node);
                        }
                    }
                    next_nodes = next_next_nodes;
                    next_next_nodes = HashSet::new();
                }

                let mut leaf = graph.leaf_nodes().into_iter().next().unwrap();
                let mut result = vec![leaf];
                let total_length = table.get(&leaf).unwrap().1;
                while let Some((source, _)) = table.get(&leaf) {
                    result.push(*source);
                    leaf = *source;
                }
                result.reverse();
                (result, total_length)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut graph = Graph::new();
        graph
            .add(&[1, 2], 9)
            .add(&[1, 3], 7)
            .add(&[1, 4], 3)
            .add(&[1, 5], 2)
            .add(&[2, 6], 4)
            .add(&[2, 7], 2)
            .add(&[2, 8], 1)
            .add(&[3, 6], 2)
            .add(&[3, 7], 7)
            .add(&[4, 8], 11)
            .add(&[5, 7], 11)
            .add(&[5, 8], 8)
            .add(&[6, 9], 6)
            .add(&[6, 10], 5)
            .add(&[7, 9], 4)
            .add(&[7, 10], 3)
            .add(&[8, 10], 5)
            .add(&[8, 11], 6)
            .add(&[9, 12], 4)
            .add(&[10, 12], 2)
            .add(&[11, 12], 5);

        assert_eq!(Solution::shortest_path(graph), (vec![1, 3, 6, 10, 12], 16));
    }
}
