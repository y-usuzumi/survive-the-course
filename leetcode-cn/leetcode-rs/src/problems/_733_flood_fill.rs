// https://leetcode-cn.com/problems/flood-fill/
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut pixels_to_process = VecDeque::from([(sr, sc)]);
        let mut visited = vec![vec![false; image[0].len()]; image.len()];
        let starting_color = image[sr as usize][sc as usize];
        while let Some((r, c)) = pixels_to_process.pop_front() {
            if visited[r as usize][c as usize] {
                continue;
            }
            if image[r as usize][c as usize] != starting_color {
                visited[r as usize][c as usize] = true;
                continue;
            }
            image[r as usize][c as usize] = new_color;
            visited[r as usize][c as usize] = true;
            if r > 0 {
                pixels_to_process.push_back((r-1, c));
            }
            if r < (image.len() - 1) as i32 {
                pixels_to_process.push_back((r+1, c));
            }
            if c > 0 {
                pixels_to_process.push_back((r, c-1));
            }
            if c < (image[0].len() - 1) as i32 {
                pixels_to_process.push_back((r, c+1));
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let image = vec![vec![1,1,1],vec![1,1,0],vec![1,0,1]];
        assert_eq!(Solution::flood_fill(image, 1, 1, 2), vec![[2,2,2],[2,2,0],[2,0,1]]);
    }

    #[test]
    fn test_2() {
        let image = vec![vec![0,0,0],vec![0,0,0]];
        assert_eq!(Solution::flood_fill(image, 0, 0, 2), vec![vec![2,2,2],vec![2,2,2]]);
    }
}