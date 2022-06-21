/*
 * @lc app=leetcode id=32 lang=typescript
 *
 * [32] Longest Valid Parentheses
 *
 * https://leetcode.com/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (30.36%)
 * Likes:    5821
 * Dislikes: 201
 * Total Accepted:    401.7K
 * Total Submissions: 1.3M
 * Testcase Example:  '"(()"'
 *
 * Given a string containing just the characters '(' and ')', find the length
 * of the longest valid (well-formed) parentheses substring.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()".
 *
 *
 * Example 2:
 *
 *
 * Input: s = ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()".
 *
 *
 * Example 3:
 *
 *
 * Input: s = ""
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= s.length <= 3 * 10^4
 * s[i] is '(', or ')'.
 *
 *
 */

// @lc code=start
function longestValidParentheses(s: string): number {
  let l = 0,
    r = 0,
    p = 0,
    max = 0,
    result = 0;
  for (let i = 0; i < s.length; i++) {
    (p = 0), (l = 0), (r = 0), (max = 0);
    while (l >= r && i + p < s.length) {
      if (s[i + p] === "(") l++, p++;
      if (s[i + p] === ")") r++, p++;
      if (l === r && p > max) max = p;
    }
    if (max > result) result = max;
    // console.log(max);
  }
  return result;
}
// @lc code=end
longestValidParentheses(")()())");
