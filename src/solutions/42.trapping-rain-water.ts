/*
 * @lc app=leetcode id=42 lang=typescript
 *
 * [42] Trapping Rain Water
 *
 * https://leetcode.com/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (55.46%)
 * Likes:    16728
 * Dislikes: 236
 * Total Accepted:    1M
 * Total Submissions: 1.8M
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * Given n non-negative integers representing an elevation map where the width
 * of each bar is 1, compute how much water it can trap after raining.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 * Explanation: The above elevation map (black section) is represented by array
 * [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue
 * section) are being trapped.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [4,2,0,3,2,5]
 * Output: 9
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 1 <= n <= 2 * 10^4
 * 0 <= height[i] <= 10^5
 *
 *
 */

// @lc code=start
function trap(height: number[]): number {
  let i = 0,
    j = height.length - 1,
    h = 0,
    base = 0;
  while (i < j) {
    if (height[i] <= height[j]) {
      base = height[i];
      while (height[i] <= base && i < j) {
        h += base - height[i];
        i++;
      }
    } else {
      base = height[j];
      while (height[j] <= base && j > i) {
        h += base - height[j];
        j--;
      }
    }
  }
  return h;
}
// @lc code=end
