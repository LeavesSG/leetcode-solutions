/*
 * @lc app=leetcode id=5 lang=typescript
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (31.08%)
 * Likes:    12540
 * Dislikes: 763
 * Total Accepted:    1.4M
 * Total Submissions: 4.5M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *
 * Example 3:
 *
 *
 * Input: s = "a"
 * Output: "a"
 *
 *
 * Example 4:
 *
 *
 * Input: s = "ac"
 * Output: "a"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consist of only digits and English letters.
 *
 *
 */

// @lc code=start
function longestPalindrome(s: string): string {
  let i = 0,
    j = 0,
    longest = "";
  for (i = 0; i < s.length; i++) {
    j = 0;
    while (s[i - j] && s[i - j] === s[i + j]) j++;
    if (longest.length < j * 2 - 1) longest = s.substring(i - j + 1, i + j);
  }
  for (i = 0; i < s.length - 1; i++) {
    j = 0;
    while (s[i - j] && s[i] === s[i + 1] && s[i - j] === s[i + 1 + j]) j++;
    if (longest.length < j * 2) longest = s.substring(i - j + 1, i + j + 1);
  }
  return longest;
}
// @lc code=end
