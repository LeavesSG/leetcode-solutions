/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
&& */
use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let width: f64 = (matrix[0].len() as f64) / 2.0;
        println!("{}", width)
    }
}
// @lc code=end

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
}
