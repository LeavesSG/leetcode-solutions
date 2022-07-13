/*
 * @lc app=leetcode id=64 lang=rust
 *
 * [64] Minimum Path Sum
 *
 * https://leetcode.com/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (58.91%)
 * Likes:    6878
 * Dislikes: 94
 * Total Accepted:    676.7K
 * Total Submissions: 1.1M
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top
 * left to bottom right, which minimizes the sum of all numbers along its
 * path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 *
 * Example 1:
 *
 *
 * Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
 * Output: 7
 * Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
 *
 *
 * Example 2:
 *
 *
 * Input: grid = [[1,2,3],[4,5,6]]
 * Output: 12
 *
 *
 *
 * Constraints:
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 200
 * 0 <= grid[i][j] <= 100
 *
 *
 */

// impl Solution{
//     pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//         let m = grid.len();
//         let n = grid[0].len();
//         let mut min_path: Vec<i32> = Vec::from([]);
//         fn count_from(grid: &Vec<Vec<i32>>, row: usize, col1: usize, col2: usize) -> i32 {
//             let mut count = 0;
//             for i in col1..col2 + 1 {
//                 count += grid[row][i]
//             }
//             return count;
//         }
//         for j in 0..n {
//             min_path.push(count_from(&grid, 0, 0, j));
//         }
//         for i in 1..m {
//             for j in (0..n).rev() {
//                 // get min_path j
//                 min_path[j] = min_path[j] + grid[i][j];
//                 for k in (0..j).rev() {
//                     let new_path = count_from(&grid, i, k, j);

//                     if (min_path[k] + new_path) < min_path[j] {
//                         min_path[j] = min_path[k] + new_path;
//                     }
//                 }
//             }
//         }
//         return min_path[n - 1];
//     }
// }

use crate::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        fn min(i: i32, j: i32) -> i32 {
            if i < j {
                return i;
            }
            return j;
        }
        let mut dp = grid.clone();
        for i in 0..m {
            for j in 0..n {
                if i == 0 {
                    if j == 0 {
                        dp[i][j] = grid[i][j];
                    } else {
                        dp[i][j] = grid[i][j] + dp[i][j - 1]
                    }
                } else {
                    if j == 0 {
                        dp[i][j] = grid[i][j] + dp[i - 1][j]
                    } else {
                        dp[i][j] = grid[i][j] + min(dp[i - 1][j], dp[i][j - 1])
                    }
                }
            }
        }
        return dp[m - 1][n - 1];
    }
}
// @lc code=end

#[test]
pub fn test() {
    let grid = Vec::from([
        Vec::from([1, 1, 1]),
        Vec::from([1, 2, 1]),
        Vec::from([1, 1, 3]),
    ]);
    assert!(Solution::min_path_sum(grid) == 7);
}
