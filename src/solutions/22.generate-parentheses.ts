/*
 * @lc app=leetcode id=22 lang=typescript
 *
 * [22] Generate Parentheses
 *
 * https://leetcode.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (69.19%)
 * Likes:    11628
 * Dislikes: 457
 * Total Accepted:    977.1K
 * Total Submissions: 1.4M
 * Testcase Example:  '3'
 *
 * Given n pairs of parentheses, write a function to generate all combinations
 * of well-formed parentheses.
 *
 *
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 8
 *
 *
 */

// @lc code=start
function generateParenthesis(n: number): string[] {
  if (n <= 0) return [];
  const unresolved = [[1]],
    resolved = [];
  let i: number, pattern: number[], string: string, last_num: number;

  // main loop
  while (unresolved.length > 0) {
    pattern = unresolved.pop() || [];
    if (pattern.length === n) resolved.push(pattern);
    else {
      last_num = pattern[pattern.length - 1];
      for (i = 1; i <= last_num + 1; i++) {
        unresolved.push([...pattern, i]);
      }
    }
  }
  // interpret
  return resolved.map((p) => {
    string = "";
    let i = 0;
    for (let j = 0; j < n; j++) {
      string += ")".repeat(i - p[j] + 1);
      string += "(";
      i = p[j];
    }
    string += ")".repeat(i);
    return string;
  });
}

// @lc code=end
