/*
 * @lc app=leetcode id=3 lang=typescript
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (32.00%)
 * Likes:    16418
 * Dislikes: 792
 * Total Accepted:    2.4M
 * Total Submissions: 7.5M
 * Testcase Example:  '"abcabcbb"'
 *
 * Given a string s, find the length of the longest substring without repeating
 * characters.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not
 * a substring.
 *
 *
 * Example 4:
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
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 *
 *
 */

// @lc code=start
function lengthOfLongestSubstring(s: string): number {
  let L = 0;
  for (let j = 0; j < s.length; j++) {
    let i = j - 1;
    let string = s[j];
    while (!string.includes(s[i]) && i >= 0) {
      string = string + s[i];
      i--;
    }
    if (string.length > L) L = string.length;
  }
  return L;
}

// const result = lengthOfLongestSubstring("dasdasdsadad");
// @lc code=end
