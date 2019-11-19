struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let (mut lidx, mut ridx) = (0, nums.len());
        let bound_target = nums[0];
        if target == bound_target {
            return 0;
        }
        while lidx < ridx {
            let midx = (lidx + ridx) / 2;
            if nums[midx] > bound_target {
                lidx = midx + 1;
            } else {
                ridx = midx;
            }
        }
        if target > bound_target {
            ridx = lidx;
            lidx = 0;
        } else {
            ridx = nums.len();
        }
        while lidx < ridx {
            let midx = (lidx + ridx) / 2;
            if nums[midx] > target {
                ridx = midx;
            } else if nums[midx] < target {
                lidx = midx + 1;
            } else {
                lidx = midx;
                break;
            }
        }
        if lidx >= nums.len() || nums[lidx] != target {
            return -1;
        }
        return lidx as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
    }
}
