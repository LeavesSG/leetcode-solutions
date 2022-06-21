/*
 * @lc app=leetcode id=64 lang=typescript
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

// @lc code=start
function minPathSum(grid: number[][]): number {
  function mPS(r: number, c: number): number {
    let s = 0;
    if (r === 0) {
      for (let i = 0; i <= c; i++) s += grid[r][i];
      return s;
    } else if (c === 0) {
      for (let i = 0; i <= r; i++) s += grid[i][c];
      return s;
    }
    return Math.min(mPS(r - 1, c) + grid[r][c], mPS(r, c - 1) + grid[r][c]);
  }
  const r = grid.length - 1;
  const c = grid[0]?.length - 1 || 0;
  const res = mPS(r, c);
  return res;
}
// @lc code=end
minPathSum([
  [1, 3, 1],
  [1, 5, 1],
  [4, 2, 1],
]);
