struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            mut n => {
                let (mut fst, mut snd, mut trd) = (0, 1, 1);
                let mut snd_and_trd = 2;
                while n > 2 {
                    let next_trd = fst + snd_and_trd;
                    fst = snd;
                    snd = trd;
                    trd = next_trd;
                    snd_and_trd = snd + trd;
                    n -= 1;
                }
                trd
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}