/*
 * @lc app=leetcode id=20 lang=typescript
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (40.60%)
 * Likes:    11132
 * Dislikes: 469
 * Total Accepted:    2M
 * Total Submissions: 4.8M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */

// @lc code=start
function isValid(s: string): boolean {
  let stack = "";
  let p = 0;
  for (let i = 0; i < s.length; i++) {
    const diff = s.charCodeAt(i) - stack.charCodeAt(p - 1);
    if (p >= 0 && (diff === 1 || diff === 2)) {
      stack = stack.substring(0, p-- - 1);
    } else {
      stack += s[i];
      p++;
    }
  }
  return stack.length === 0;
}
// @lc code=end
