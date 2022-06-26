/*
 * @lc app=leetcode id=46 lang=typescript
 *
 * [46] Permutations
 *
 * https://leetcode.com/problems/permutations/description/
 *
 * algorithms
 * Medium (71.40%)
 * Likes:    9367
 * Dislikes: 175
 * Total Accepted:    1.1M
 * Total Submissions: 1.5M
 * Testcase Example:  '[1,2,3]'
 *
 * Given an array nums of distinct integers, return all the possible
 * permutations. You can return the answer in any order.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 6
 * -10 <= nums[i] <= 10
 * All the integers of nums are unique.
 *
 *
 */

// @lc code=start
function permute(nums: number[]): number[][] {
  function nextPermutation(nums: number[]): void {
    let L = nums.length - 1;
    let i = L,
      j = L,
      temp: number;
    function exch(a: number, b: number) {
      temp = nums[a];
      nums[a] = nums[b];
      nums[b] = temp;
    }
    while (i > 0 && nums[i - 1] >= nums[i]) i--;
    if (i === 0) while (i < j) exch(i, j), i++, j--;
    else {
      while (nums[j] <= nums[i - 1]) j--;
      exch(i - 1, j);
      j = L;
      while (i < j) exch(i, j), i++, j--;
    }
  }
  let L = 1;
  for (let i = 1; i <= nums.length; i++) L *= i;
  const result = [];
  let arr = [...nums];
  for (let i = 0; i < L; i++) {
    arr = [...arr];
    nextPermutation(arr);
    result.push(arr);
  }
  return result;
}
// @lc code=end
console.log(permute([1, 2, 3]));
