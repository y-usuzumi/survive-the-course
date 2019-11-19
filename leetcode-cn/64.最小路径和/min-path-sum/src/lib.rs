struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0
        }
        if grid[0].len() == 0 {
            return 0
        }

        {
            let row = &mut grid[0];
            let mut cidx = 1;
            while cidx < row.len() {
                row[cidx] += row[cidx-1];
                cidx += 1;
            }
        }

        let mut ridx = 1;
        let mut cidx = 1;
        while ridx < grid.len() {
            cidx = 1;
            grid[ridx][0] += grid[ridx-1][0];
            while cidx < grid[ridx].len() {
                grid[ridx][cidx] += grid[ridx][cidx-1].min(grid[ridx-1][cidx]);
                cidx += 1
            }
            ridx += 1;
        }

        *grid.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![
            vec![1,3,1],
            vec![1,5,1],
            vec![4,2,1]
        ];
        let sum = Solution::min_path_sum(grid);
        assert_eq!(sum, 7);
    }
}
