/*
 * @lc app=leetcode id=41 lang=typescript
 *
 * [41] First Missing Positive
 *
 * https://leetcode.com/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (35.57%)
 * Likes:    8554
 * Dislikes: 1250
 * Total Accepted:    628.5K
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,2,0]'
 *
 * Given an unsorted integer array nums, return the smallest missing positive
 * integer.
 *
 * You must implement an algorithm that runs in O(n) time and uses constant
 * extra space.
 *
 *
 * Example 1:
 * Input: nums = [1,2,0]
 * Output: 3
 * Example 2:
 * Input: nums = [3,4,-1,1]
 * Output: 2
 * Example 3:
 * Input: nums = [7,8,9,11,12]
 * Output: 1
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 5 * 10^5
 * -2^31 <= nums[i] <= 2^31 - 1
 *
 *
 */

// @lc code=start
function firstMissingPositive(nums: number[]): number {
  // filter negative
  nums.push(nums[0]);
  let i = 0,
    j = nums.length - 1,
    temp: number;
  while (i < j) {
    if (nums[i] <= 0) {
      temp = nums[j];
      nums[j--] = nums[i];
      nums[i] = temp;
    } else {
      i++;
    }
  }

  // build heap

  return 0;
}
// @lc code=end
