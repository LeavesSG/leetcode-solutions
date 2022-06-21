/*
 * @lc app=leetcode id=75 lang=typescript
 *
 * [75] Sort Colors
 *
 * https://leetcode.com/problems/sort-colors/description/
 *
 * algorithms
 * Medium (54.04%)
 * Likes:    8918
 * Dislikes: 388
 * Total Accepted:    938.1K
 * Total Submissions: 1.7M
 * Testcase Example:  '[2,0,2,1,1,0]'
 *
 * Given an array nums with n objects colored red, white, or blue, sort them
 * in-place so that objects of the same color are adjacent, with the colors in
 * the order red, white, and blue.
 *
 * We will use the integers 0, 1, and 2 to represent the color red, white, and
 * blue, respectively.
 *
 * You must solve this problem without using the library's sort function.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 *
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 300
 * nums[i] is either 0, 1, or 2.
 *
 *
 *
 * Follow up: Could you come up with a one-pass algorithm using only constant
 * extra space?
 *
 */

// @lc code=start
/**
 Do not return anything, modify nums in-place instead.
 */
function sortColors(nums: number[]): void {
  let i = 0,
    j = nums.length - 1;
  while (i < j) {
    while (nums[i] <= 1 && i < nums.length - 1) i++;
    while (nums[j] > 1 && j > 0) j--;
    if (i < j) {
      const temp = nums[i];
      nums[i] = nums[j];
      nums[j] = temp;
    }
  }
}
// @lc code=end
sortColors([0, 1, 2, 1, 0, 1, 2, 2, 0, 0, 1, 2, 1, 0]);
