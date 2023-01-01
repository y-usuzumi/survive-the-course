// https://leetcode.com/problems/cat-and-mouse/

use std::collections::{HashSet, VecDeque};

use self::Result::*;

pub struct Solution;

// (cat position, mouse position, next)
// where next == 1 if mouse, 0 if cat
type State = (usize, usize, usize);

#[derive(Copy, Clone, PartialEq)]
enum Result {
    Draw,
    MouseWin,
    CatWin,
}

struct Game {
    nodes: usize,
    edges: Vec<Vec<i32>>,
    states: Vec<Vec<Vec<Result>>>,
    remaining_edges: Vec<Vec<Vec<usize>>>,
}

impl Game {
    fn new(graph: Vec<Vec<i32>>) -> Self {
        let len = graph.len();
        // Row: cat positions
        // Col: mouse positions
        // Height: 0: next is cat; 1: next is mouse
        let mut remaining_edges = vec![vec![vec![0; 2]; len]; len];
        for cat_edge in 0..len {
            for mouse_edge in 0..len {
                // Cat cannot be at node 0. Exclude it from possible edges
                remaining_edges[cat_edge][mouse_edge][0] = graph[cat_edge]
                    .iter()
                    .filter(|cat_edge| **cat_edge != 0)
                    .count();
                remaining_edges[cat_edge][mouse_edge][1] = graph[mouse_edge].len();
            }
        }

        Self {
            nodes: len,
            edges: graph,
            // Default result is Draw. We will be updating the states to CatWin
            // or MouseWin. Some states will never be touched and remain Draw.
            states: vec![vec![vec![Draw; 2]; len]; len],
            remaining_edges,
        }
    }

    // Returns next nodes with no in-degree. Used for topological sorting
    fn update_remaining_edges(&mut self, cat: usize, mouse: usize, next: usize) -> Vec<State> {
        let mut result = vec![];
        if next == 1 {
            // Next is mouse, prev is cat
            for prev_cat in self.edges[cat].clone() {
                // Cat cannot be at node 0
                if prev_cat == 0 {
                    continue;
                }
                let remaining_edges = &mut self.remaining_edges[prev_cat as usize][mouse][0];
                // Some nodes may have been visited before, including the nodes
                // whose initial results are determined, and also those whose
                // results have been deduced by only a single child. We must not
                // visit them again or we will get subtract overflow.
                if *remaining_edges > 0 {
                    *remaining_edges -= 1;
                    if *remaining_edges == 0 {
                        result.push((prev_cat as usize, mouse, 0));
                    }
                }
            }
        } else {
            // Next is cat, prev is mouse
            for prev_mouse in self.edges[mouse].clone() {
                let remaining_edges = &mut self.remaining_edges[cat][prev_mouse as usize][1];
                if *remaining_edges > 0 {
                    *remaining_edges -= 1;
                    if *remaining_edges == 0 {
                        result.push((cat, prev_mouse as usize, 1));
                    }
                }
            }
        }

        return result;
    }

    fn judge(&mut self) -> Result {
        let mut q = VecDeque::new();
        // Populate results that can be determined immediately
        for idx in 1..self.nodes {
            // Mouse reaches hole
            self.remaining_edges[idx][0][0] = 0;
            self.remaining_edges[idx][0][1] = 0;
            q.push_back((idx, 0, 0));
            q.push_back((idx, 0, 1));
            self.states[idx][0][0] = MouseWin;
            self.states[idx][0][1] = MouseWin;

            // Cat catches mouse
            self.remaining_edges[idx][idx][0] = 0;
            self.remaining_edges[idx][idx][1] = 0;
            q.push_back((idx, idx, 0));
            q.push_back((idx, idx, 1));
            self.states[idx][idx][0] = CatWin;
            self.states[idx][idx][1] = CatWin;
        }

        while let Some((cat, mouse, next)) = q.pop_front() {
            if next == 1 {
                // Next is mouse, current step is taken by cat
                match self.states[cat][mouse][next] {
                    CatWin => {
                        for prev_cat in self.edges[cat].clone() {
                            if prev_cat == 0 {
                                continue;
                            }
                            // If a state is not Draw, it is already visited.
                            if self.states[prev_cat as usize][mouse][0] != Draw {
                                continue;
                            }
                            // This state can be determined immediately.
                            // Therefore we do not need to wait for all sibling
                            // nodes to be visited.
                            self.remaining_edges[prev_cat as usize][mouse][0] = 0;
                            // If the current result is CatWin and it is Mouse's
                            // turn, then the previous turn was done by Cat, and
                            // apparently Cat can win the game simply by moving
                            // from `prev_cat` to the `cat`.
                            self.states[prev_cat as usize][mouse][0] = CatWin;
                            q.push_back((prev_cat as usize, mouse, 0));
                        }
                    }
                    MouseWin => {
                        for prev_node in self.update_remaining_edges(cat, mouse, next) {
                            // We do NOT decrement remaining_edges of a state
                            // when its subsequent state is a Draw. And we will
                            // immediately set remaining edges to 0 if it is a
                            // CatWin (in the arm above). Therefore, it is ONLY
                            // possible that remaining_edges reach 0 here when
                            // its children states are ALL MouseWin. When this
                            // happens, there is no way for Cat to win.
                            self.states[prev_node.0][prev_node.1][prev_node.2] = MouseWin;
                            q.push_back(prev_node);
                        }
                    }
                    Draw => {}
                }
            } else {
                // Same as in the Cat branch but reversed
                match self.states[cat][mouse][0] {
                    MouseWin => {
                        for prev_mouse in self.edges[mouse].clone() {
                            if self.states[cat][prev_mouse as usize][1] != Draw {
                                continue;
                            }
                            self.remaining_edges[cat][prev_mouse as usize][1] = 0;
                            self.states[cat][prev_mouse as usize][1] = MouseWin;
                            q.push_back((cat, prev_mouse as usize, 1));
                        }
                    }
                    CatWin => {
                        for prev_node in self.update_remaining_edges(cat, mouse, next) {
                            self.states[prev_node.0][prev_node.1][prev_node.2] = CatWin;
                            q.push_back(prev_node);
                        }
                    }
                    Draw => {}
                }
            }
        }

        return self.states[2][1][1];
    }
}

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let mut game = Game::new(graph);
        match game.judge() {
            CatWin => 2,
            MouseWin => 1,
            Draw => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let graph = vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(Solution::cat_mouse_game(graph), 0);
    }

    #[test]
    fn test_2() {
        let graph = vec![
            vec![5, 6],
            vec![3, 4],
            vec![6],
            vec![1, 4, 5],
            vec![1, 3, 5],
            vec![0, 3, 4, 6],
            vec![0, 2, 5],
        ];
        assert_eq!(Solution::cat_mouse_game(graph), 2);
    }

    #[test]
    fn test_3() {
        let graph = vec![
            vec![3, 4, 6, 7, 9, 15, 16, 18],
            vec![4, 5, 8, 19],
            vec![3, 4, 6, 9, 17, 18],
            vec![0, 2, 11, 15],
            vec![0, 1, 10, 6, 2, 12, 14, 16],
            vec![1, 10, 7, 9, 15, 17, 18],
            vec![0, 10, 4, 7, 9, 2, 11, 12, 13, 14, 15, 17, 19],
            vec![0, 10, 5, 6, 9, 16, 17],
            vec![1, 9, 14, 15, 16, 19],
            vec![0, 10, 5, 6, 7, 8, 2, 11, 13, 15, 16, 17, 18],
            vec![4, 5, 6, 7, 9, 18],
            vec![3, 6, 9, 12, 19],
            vec![4, 6, 11, 15, 17, 19],
            vec![6, 9, 15, 17, 18, 19],
            vec![4, 6, 8, 15, 19],
            vec![0, 3, 5, 6, 8, 9, 12, 13, 14, 16, 19],
            vec![0, 4, 7, 8, 9, 15, 17, 18, 19],
            vec![5, 6, 7, 9, 2, 12, 13, 16],
            vec![0, 10, 5, 9, 2, 13, 16],
            vec![1, 6, 8, 11, 12, 13, 14, 15, 16],
        ];
        assert_eq!(Solution::cat_mouse_game(graph), 2);
    }
}
