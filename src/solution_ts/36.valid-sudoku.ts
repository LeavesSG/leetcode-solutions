/*
 * @lc app=leetcode id=36 lang=typescript
 *
 * [36] Valid Sudoku
 *
 * https://leetcode.com/problems/valid-sudoku/description/
 *
 * algorithms
 * Medium (54.23%)
 * Likes:    4440
 * Dislikes: 670
 * Total Accepted:    667.3K
 * Total Submissions: 1.2M
 * Testcase Example:  '[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]'
 *
 * Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be
 * validated according to the following rules:
 *
 *
 * Each row must contain the digits 1-9 without repetition.
 * Each column must contain the digits 1-9 without repetition.
 * Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9
 * without repetition.
 *
 *
 * Note:
 *
 *
 * A Sudoku board (partially filled) could be valid but is not necessarily
 * solvable.
 * Only the filled cells need to be validated according to the mentioned
 * rules.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: board =
 * [["5","3",".",".","7",".",".",".","."]
 * ,["6",".",".","1","9","5",".",".","."]
 * ,[".","9","8",".",".",".",".","6","."]
 * ,["8",".",".",".","6",".",".",".","3"]
 * ,["4",".",".","8",".","3",".",".","1"]
 * ,["7",".",".",".","2",".",".",".","6"]
 * ,[".","6",".",".",".",".","2","8","."]
 * ,[".",".",".","4","1","9",".",".","5"]
 * ,[".",".",".",".","8",".",".","7","9"]]
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: board =
 * [["8","3",".",".","7",".",".",".","."]
 * ,["6",".",".","1","9","5",".",".","."]
 * ,[".","9","8",".",".",".",".","6","."]
 * ,["8",".",".",".","6",".",".",".","3"]
 * ,["4",".",".","8",".","3",".",".","1"]
 * ,["7",".",".",".","2",".",".",".","6"]
 * ,[".","6",".",".",".",".","2","8","."]
 * ,[".",".",".","4","1","9",".",".","5"]
 * ,[".",".",".",".","8",".",".","7","9"]]
 * Output: false
 * Explanation: Same as Example 1, except with the 5 in the top left corner
 * being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it
 * is invalid.
 *
 *
 *
 * Constraints:
 *
 *
 * board.length == 9
 * board[i].length == 9
 * board[i][j] is a digit 1-9 or '.'.
 *
 *
 */

// @lc code=start
function isValidSudoku(board: string[][]): boolean {
  let i, j;
  let ht: any = [];

  // row validation
  for (i = 0; i < 9; i++) {
    ht = [];
    for (let j = 0; j < 9; j++) {
      if (board[i][j] !== ".") {
        if (ht[board[i][j]]) return false;
        ht[board[i][j]] = 1;
      }
    }
  }
  // column validation
  for (i = 0; i < 9; i++) {
    ht = [];
    for (let j = 0; j < 9; j++) {
      if (board[j][i] !== ".") {
        if (ht[board[j][i]]) return false;
        ht[board[j][i]] = 1;
      }
    }
  }
  // small block validation
  for (i = 0; i < 9; i += 3) {
    for (j = 0; j < 9; j += 3) {
      ht = [];
      for (let h = 0; h < 9; h++) {
        let [m, n] = [i + Math.floor(h / 3), j + (h % 3)];
        if (board[m][n] !== ".") {
          if (ht[board[m][n]]) return false;
          ht[board[m][n]] = 1;
        }
      }
    }
  }

  return true;
}
// @lc code=end
