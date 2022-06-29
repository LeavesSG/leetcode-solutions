/*
 * @lc app=leetcode id=15 lang=typescript
 *
 * [15] 3Sum
 *
 * https://leetcode.com/problems/3sum/description/
 *
 * algorithms
 * Medium (30.46%)
 * Likes:    15780
 * Dislikes: 1513
 * Total Accepted:    1.7M
 * Total Submissions: 5.7M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j],
 * nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] +
 * nums[k] == 0.
 *
 * Notice that the solution set must not contain duplicate triplets.
 *
 *
 * Example 1:
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Example 2:
 * Input: nums = []
 * Output: []
 * Example 3:
 * Input: nums = [0]
 * Output: []
 *
 *
 * Constraints:
 *
 *
 * 0 <= nums.length <= 3000
 * -10^5 <= nums[i] <= 10^5
 *
 *
 */

// @lc code=start
const BS1 = (
  length: number,
  compare: (index: number) => number,
  strict = true
): number => {
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
function threeSum(nums: number[]): number[][] {
  nums = nums.sort((a, b) => b - a);
  const result: number[][] = [];
  for (let i = 0; i < nums.length - 3; i++) {
    for (let j = i + 1; j < nums.length - 2; j++) {
      const found = BS1(
        nums.length - 1,
        (index) => nums[i] + nums[j] + nums[index]
      );
      if (found && found !== i && found !== j) result.push([i, j, found]);
    }
  }
  return result;
}
// @lc code=end
