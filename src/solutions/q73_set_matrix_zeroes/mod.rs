/*
 * @lc app=leetcode id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
 *
 * https://leetcode.com/problems/set-matrix-zeroes/description/
 *
 * algorithms
 * Medium (47.73%)
 * Likes:    7398
 * Dislikes: 499
 * Total Accepted:    714.9K
 * Total Submissions: 1.5M
 * Testcase Example:  '[[1,1,1],[1,0,1],[1,1,1]]'
 *
 * Given an m x n integer matrix matrix, if an element is 0, set its entire row
 * and column to 0's.
 *
 * You must do it in place.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: [[1,0,1],[0,0,0],[1,0,1]]
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 * Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[0].length
 * 1 <= m, n <= 200
 * -2^31 <= matrix[i][j] <= 2^31 - 1
 *
 *
 *
 * Follow up:
 *
 *
 * A straightforward solution using O(mn) space is probably a bad idea.
 * A simple improvement uses O(m + n) space, but still not the best
 * solution.
 * Could you devise a constant space solution?
 *
 *
 */

use crate::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        if m == 0 {
            return;
        }
        let n = matrix[0].len();
        let mut queue: Vec<(usize, usize)> = Vec::from([]);
        for (ri, row) in matrix.iter().enumerate() {
            for (ci, item) in row.iter().enumerate() {
                if *item == 0 {
                    queue.push((ri, ci));
                }
            }
        }
        for (row, col) in queue {
            for row in matrix.iter_mut() {
                row[col] = 0
            }
            for j in 0..n {
                matrix[row][j] = 0
            }
        }
    }
}
// @lc code=end

#[test]
pub fn test() {
    use crate::{prettyprint, vecnd};
    let mut grid = vecnd!([1, 1, 1], [1, 0, 1], [1, 1, 3]);
    Solution::set_zeroes(&mut grid);
    println!("The grid after 'Set Zero' is: ");
    prettyprint!(&grid);
    assert_eq!(grid, vecnd!([1, 0, 1], [0, 0, 0], [1, 0, 3]));
}
