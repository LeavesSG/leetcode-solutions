/*
 * @lc app=leetcode id=120 lang=rust
 *
 * [120] Triangle
 */
use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let max_depth = triangle.len() - 1;
        let mut dp: Vec<Vec<i32>> = triangle.clone();
        let mut depth = 0;
        let mut index = 0;
        let max_index = |depth: usize| &triangle[depth].len() - 1;

        while depth <= max_depth && index <= triangle[max_depth].len() {
            let curr = triangle[depth][index];
            let min: i32;
            match (depth, index) {
                (0, 0) => min = curr,
                (depth, 0) => {
                    min = dp[depth - 1][0] + curr;
                }
                (depth, index) if (index == max_index(depth)) => {
                    min = dp[depth - 1][index - 1] + curr;
                }
                _ => min = dp[depth - 1][index - 1].min(dp[depth - 1][index]) + curr,
            }
            dp[depth][index] = min;
            if index == max_index(depth) {
                depth += 1;
                index = 0
            } else {
                index += 1
            }
        }
        let c = &dp[max_depth];
        return *c.iter().min().expect("");
    }
}
// @lc code=end

#[test]
fn test() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let result = Solution::minimum_total(triangle);
    assert_eq!(result, 11);
}
