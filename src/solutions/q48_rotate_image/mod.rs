/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
&&& */
use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let height = matrix.len() - 1;
        let width = matrix[0].len() - 1;
        let (move_width, move_height) = (
            (((width + 1) as f64) / 2.0).ceil() as usize,
            (((height + 1) as f64) / 2.0).floor() as usize,
        );
        for m in 0..move_width {
            for n in 0..move_height {
                let temp = matrix[m][n];
                matrix[m][n] = matrix[height - n][m];
                matrix[height - n][m] = matrix[height - m][width - n];
                matrix[height - m][width - n] = matrix[n][width - m];
                matrix[n][width - m] = temp;
            }
        }
    }
}
// @lc code=end

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]])
}
