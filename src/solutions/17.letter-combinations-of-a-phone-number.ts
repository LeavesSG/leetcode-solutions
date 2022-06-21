/*
 * @lc app=leetcode id=17 lang=typescript
 *
 * [17] Letter Combinations of a Phone Number
 *
 * https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (52.80%)
 * Likes:    8870
 * Dislikes: 627
 * Total Accepted:    1.1M
 * Total Submissions: 2M
 * Testcase Example:  '"23"'
 *
 * Given a string containing digits from 2-9 inclusive, return all possible
 * letter combinations that the number could represent. Return the answer in
 * any order.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given
 * below. Note that 1 does not map to any letters.
 *
 *
 *
 *
 * Example 1:
 *
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 *
 * Example 2:
 *
 *
 * Input: digits = ""
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= digits.length <= 4
 * digits[i] is a digit in the range ['2', '9'].
 *
 *
 */

// @lc code=start
function letterCombinations(digits: string): string[] {
  const mapping = [
    [],
    [],
    ["a", "b", "c"],
    ["d", "e", "f"],
    ["g", "h", "i"],
    ["j", "k", "l"],
    ["m", "n", "o"],
    ["p", "q", "r", "s"],
    ["t", "u", "v"],
    ["w", "x", "y", "z"],
  ];
  const result = [];
  const pointer = Array.from({ length: digits.length }, () => 0);
  let i = 0;
  while (pointer[i] < mapping[Number(digits[i])]?.length || i < pointer.length) {
    const string = pointer
      .map((p: number, idx: number) => mapping[Number(digits[idx])][p])
      .join("");
    result.push(string);
    console.log(...pointer, string);
    // increment
    if (pointer[i] < mapping[Number(digits[i])].length - 1) {
      pointer[i]++;
    } else {
      i++;
      for (let j = 0; j < i; j++) {
        pointer[j] = 0;
      }
    }
  }
  return result;
}
// @lc code=end
