/*
 * @lc app=leetcode id=14 lang=typescript
 *
 * [14] Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (38.75%)
 * Likes:    6786
 * Dislikes: 2764
 * Total Accepted:    1.4M
 * Total Submissions: 3.6M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * Write a function to find the longest common prefix string amongst an array
 * of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.n <= 200
 * 0 <= strs[i].n <= 200
 * strs[i] consists of only lower-case English letters.
 *
 *
 */

// @lc code=start
const BS = (length: number, compare: (index: number) => number, strict = true): number => {
  let lower_bound = 0;
  let upper_bound = length - 1;
  if (strict) {
    if (compare(lower_bound) > 0 || compare(upper_bound) < 0) return -1;
  } else {
    // console.log(123, lower_bound, upper_bound);
    if (compare(lower_bound) > 0) return -1;
    else if (compare(upper_bound) < 0) return upper_bound;
  }

  let pointer = Math.floor((upper_bound + lower_bound) / 2);

  while (upper_bound - lower_bound > 1) {
    if (compare(pointer) === 0) return pointer;
    if (compare(pointer) > 0) {
      upper_bound = pointer;
      pointer = Math.floor((upper_bound + lower_bound) / 2);
    } else {
      lower_bound = pointer;
      pointer = Math.floor((upper_bound + lower_bound) / 2);
    }
  }
  if (!strict) return lower_bound;
  return -1;
};

function longestCommonPrefix(strs: string[]): string {
  // 判断当pre-string长度为n时，是否strs所有字符串都包含pre-string
  const shortest = strs.reduce((a, c) => (c.length < a.length ? c : a));

  const co = (n: number) => {
    const baseString = shortest.substring(0, n + 1);
    let result = -1;
    strs.forEach((str) => {
      if (str.substring(0, n + 1) !== baseString) result = 1;
    });
    return result;
  };
  const found = BS(shortest.length, co, false);
  // console.log("found: ", found);
  if (found === -1) return "";
  else return shortest.substring(0, found + 1);
}
// @lc code=end
