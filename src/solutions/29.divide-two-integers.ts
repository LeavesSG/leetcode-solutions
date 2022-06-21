/*
 * @lc app=leetcode id=29 lang=typescript
 *
 * [29] Divide Two Integers
 *
 * https://leetcode.com/problems/divide-two-integers/description/
 *
 * algorithms
 * Medium (17.05%)
 * Likes:    2506
 * Dislikes: 8867
 * Total Accepted:    444.2K
 * Total Submissions: 2.6M
 * Testcase Example:  '10\n3'
 *
 * Given two integers dividend and divisor, divide two integers without using
 * multiplication, division, and mod operator.
 *
 * The integer division should truncate toward zero, which means losing its
 * fractional part. For example, 8.345 would be truncated to 8, and -2.7335
 * would be truncated to -2.
 *
 * Return the quotient after dividing dividend by divisor.
 *
 * Note: Assume we are dealing with an environment that could only store
 * integers within the 32-bit signed integer range: [−2^31, 2^31 − 1]. For this
 * problem, if the quotient is strictly greater than 2^31 - 1, then return 2^31
 * - 1, and if the quotient is strictly less than -2^31, then return -2^31.
 *
 *
 * Example 1:
 *
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * Explanation: 10/3 = 3.33333.. which is truncated to 3.
 *
 *
 * Example 2:
 *
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * Explanation: 7/-3 = -2.33333.. which is truncated to -2.
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= dividend, divisor <= 2^31 - 1
 * divisor != 0
 *
 *
 */

// @lc code=start
function divide(dividend: number, divisor: number): number {
  const timesN = (int: number, N: number) => {
    let num = 0;
    for (let i = 0; i < N; i++) num += int;
    return num;
  };

  const multiplication = (int: number, multiplier: number) => {
    let larger = Math.abs(int) > Math.abs(multiplier) ? int : multiplier;
    let less = Math.abs(int) < Math.abs(multiplier) ? int : multiplier;
    if (Math.abs(int) === Math.abs(multiplier)) {
      less = Math.min(int, multiplier);
      larger = Math.max(int, multiplier);
    }

    const negative = less < 0;
    const stringMulti = String(Math.abs(less));

    let temp = 0,
      digit = 0,
      result = 0;
    for (let i = stringMulti.length - 1; i >= 0; i--) {
      digit = Number(stringMulti[i]);
      if (!temp) temp = larger;
      else temp = timesN(temp, 10);
      result += timesN(temp, digit);
    }
    if (negative) return 0 - result;
    return result;
  };

  function validateResult(num: number, isNegative: boolean) {
    let result = num;
    result = isNegative ? -result : result;
    if (result > 2147483647) result = 2147483647;
    if (result < -2147483648) result = -2147483648;
    return result;
  }

  let lower_bound = 0,
    upper_bound = dividend,
    pointer = 0;
  const isNegative = divisor < 0;
  if (multiplication(divisor, lower_bound) === dividend) return lower_bound;
  if (Math.abs(multiplication(divisor, upper_bound)) === Math.abs(dividend))
    return validateResult(upper_bound, isNegative);
  while (Math.abs(upper_bound) - Math.abs(lower_bound) > 1) {
    const string = String(multiplication(upper_bound + lower_bound, 5));
    pointer = Number(string.slice(0, -1));
    // console.log(lower_bound, pointer, upper_bound, isNegative, string, multiplication(-5 + 0, 5));
    const temp = Math.abs(multiplication(divisor, pointer));
    if (temp === Math.abs(dividend)) return validateResult(pointer, isNegative);
    else if (temp > Math.abs(dividend)) {
      upper_bound = pointer;
    } else {
      lower_bound = pointer;
    }
  }
  return validateResult(lower_bound, isNegative);
}
// @lc code=end
