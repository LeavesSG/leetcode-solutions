/*
 * @lc app=leetcode id=33 lang=typescript
 *
 * [33] Search in Rotated Sorted Array
 *
 * https://leetcode.com/problems/search-in-rotated-sorted-array/description/
 *
 * algorithms
 * Medium (37.35%)
 * Likes:    12523
 * Dislikes: 834
 * Total Accepted:    1.3M
 * Total Submissions: 3.6M
 * Testcase Example:  '[4,5,6,7,0,1,2]\n0'
 *
 * There is an integer array nums sorted in ascending order (with distinct
 * values).
 *
 * Prior to being passed to your function, nums is possibly rotated at an
 * unknown pivot index k (1 <= k < nums.length) such that the resulting array
 * is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]
 * (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3
 * and become [4,5,6,7,0,1,2].
 *
 * Given the array nums after the possible rotation and an integer target,
 * return the index of target if it is in nums, or -1 if it is not in nums.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * Example 2:
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 * Example 3:
 * Input: nums = [1], target = 0
 * Output: -1
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 5000
 * -10^4 <= nums[i] <= 10^4
 * All values of nums are unique.
 * nums is an ascending array that is possibly rotated.
 * -10^4 <= target <= 10^4
 *
 *
 */

// @lc code=start
const useBinarySearch = (
  length: number,
  compare: (index: number) => number,
  strict = true
): number => {
  let lower_bound = 0;
  let upper_bound = length - 1;
  if (strict) {
    if (compare(lower_bound) > 0 || compare(upper_bound) < 0) return -1;
  } else {
    if (compare(lower_bound) > 0) return -1;
    if (compare(upper_bound) < 0) return upper_bound;
  }

  let pointer = Math.floor((upper_bound + lower_bound) / 2);

  while (lower_bound <= upper_bound) {
    if (compare(pointer) === 0) return pointer;
    if (compare(pointer) > 0) {
      upper_bound = pointer - 1;
      pointer = Math.floor((upper_bound + lower_bound) / 2);
    } else {
      lower_bound = pointer + 1;
      pointer = Math.floor((upper_bound + lower_bound) / 2);
    }
  }
  if (!strict) return lower_bound;
  return -1;
};

function search(nums: number[], target: number): number {
  if (nums.length <= 0) return -1;
  const base = nums[0];
  if (target === base) return 0;
  function compare(index: number) {
    if (target === nums[index]) return 0;
    if (target > base && nums[index] >= base && nums[index] < target) return -1;
    if (target < base && (nums[index] >= base || nums[index] < target)) return -1;
    return 1;
  }
  return useBinarySearch(nums.length, compare);
}
// @lc code=end
console.log(search([1, 3], 3));
