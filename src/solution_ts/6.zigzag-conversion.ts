/*
 * @lc app=leetcode id=6 lang=typescript
 *
 * [6] Zigzag Conversion
 *
 * https://leetcode.com/problems/zigzag-conversion/description/
 *
 * algorithms
 * Medium (40.77%)
 * Likes:    3269
 * Dislikes: 7644
 * Total Accepted:    704.2K
 * Total Submissions: 1.7M
 * Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
 * of rows like this: (you may want to display this pattern in a fixed font for
 * better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a
 * number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 *
 * Example 3:
 *
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 1 <= numRows <= 1000
 *
 *
 */

// @lc code=start
function convert(s: string, numRows: number): string {
  let c = 0;
  let l = -1;
  function __next(c: number, l: number) {
    if ((c > l || c - 1 < 0) && c + 1 < numRows) return c + 1;
    else if (c - 1 >= 0) return c - 1;
    return c;
  }
  const result: string[][] = Array.from({ length: numRows }, () => []);
  for (let i = 0; i < s.length; i++) {
    result[c]?.push(s[i]);
    const next = __next(c, l);
    l = c;
    c = next;
  }
  return result.reduce((a, c) => a.concat(c), []).join("");
}

// @lc code=end

const s = "PAYPALISHIRING";
console.log(convert(s, 3));
