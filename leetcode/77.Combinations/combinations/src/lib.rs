/// 问题中仅包含`impl Solution`部分的代码。为了让程序通过编译，定义了一个Solution结构。
pub struct Solution{}

/// LeetCode编辑器中的代码部分
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        // all_numbers为待选数值。
        // 实测发现，从all_numbers中根据下标选值比在插入结果是实时计算要快得多。
        let all_numbers = (1..n+1).collect::<Vec<i32>>();
        // 用来从all_numbers中选值的下标组合。
        // 第一个组合0到k-1显然是一个合理的组合，之后的组合会在此基础上更新。
        let mut indices = (0..k).collect::<Vec<i32>>();

        let mut result = Vec::new();

        'main: loop {
            // 使用当前下标组合选择数值插入结果集。首先插入的是初始化时的0到k-1下标组合。
            result.push(indices.iter().map(|idx| all_numbers[*idx as usize]).collect::<Vec<i32>>());
            // 更新时的原则是，下标永远要从小到大排列，优先更新最右侧下标，
            // 当当前位置的下标越界时，更新其左侧的下标(+1)，同时将当前位及右侧所有下标重新计算为从左侧下标
            // 开始依次+1，如n == 5, k == 3，某时刻下标组合为：
            // `[1, 3, 4]`
            // 则从最右侧(4)开始更新为(5),发现越界，
            // 转而更新次右侧的(3)为(4),同时将右侧所有的数值依次更新为(4+1), (4+2)...，因为只有一个值，所以更新为(5)。
            // 但是这时经过下一轮检查发现(4)已经越界，因此再转而更新左侧的(1)为(2)，同时将右侧数值依次更新为(3), (4)。
            // 以此类推。
            // FIXME: 所以这里的更新右侧下标可能会多次执行，影响效率。
            for midx in (0..k).rev() {
                indices[midx as usize] += 1;
                if indices[midx as usize] <= n - k + midx { // 位置越靠右，可选值范围就越大。
                    // 如果当前位置的下标是合理值，则右侧所有的下标都应该是合理值，因此跳出循环进入'main以便插入结果。
                    break
                }
                if midx == 0 {  // 最左侧的下标也已经不合法，结果迭代完毕。
                    break 'main
                }
                // 左侧下标的更新要等待下轮迭代，因此提前知道当前位的下一个合法值应该是左侧值+1再+1。
                let next_starting_idx = indices[(midx-1) as usize]+2;
                for nidx in midx..k {
                    indices[nidx as usize] = next_starting_idx + nidx - midx;
                }
                continue
            }
        }
        return result;
    }
}
/// /LeetCode编辑器中的代码部分

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_for_sample_test() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        )
    }
}
