/*
 * @lc app=leetcode id=38 lang=typescript
 *
 * [38] Count and Say
 *
 * https://leetcode.com/problems/count-and-say/description/
 *
 * algorithms
 * Medium (48.07%)
 * Likes:    1217
 * Dislikes: 3219
 * Total Accepted:    598K
 * Total Submissions: 1.2M
 * Testcase Example:  '1'
 *
 * The count-and-say sequence is a sequence of digit strings defined by the
 * recursive formula:
 *
 *
 * countAndSay(1) = "1"
 * countAndSay(n) is the way you would "say" the digit string from
 * countAndSay(n-1), which is then converted into a different digit string.
 *
 *
 * To determine how you "say" a digit string, split it into the minimal number
 * of groups so that each group is a contiguous section all of the same
 * character. Then for each group, say the number of characters, then say the
 * character. To convert the saying into a digit string, replace the counts
 * with a number and concatenate every saying.
 *
 * For example, the saying and conversion for digit string "3322251":
 *
 * Given a positive integer n, return the n^th term of the count-and-say
 * sequence.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 1
 * Output: "1"
 * Explanation: This is the base case.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 4
 * Output: "1211"
 * Explanation:
 * countAndSay(1) = "1"
 * countAndSay(2) = say "1" = one 1 = "11"
 * countAndSay(3) = say "11" = two 1's = "21"
 * countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 30
 *
 *
 */

// @lc code=start
function countAndSay(n: number): string {
  const countNext = (s: string) => {
    let a = 0,
      c = "";
    let result = "";
    for (let i = 0; i < s.length; i++) {
      if (s[i] === c) a++;
      else {
        if (a) result += String(a) + c;
        c = s[i];
        a = 1;
      }
    }
    if (a) result += String(a) + c;
    return result;
  };
  let init = "1"
  for (let i = 1; i < n; i++){
    init = countNext(init)
  }
  return init
}
// @lc code=end
