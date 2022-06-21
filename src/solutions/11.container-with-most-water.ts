/*
 * @lc app=leetcode id=11 lang=typescript
 *
 * [11] Container With Most Water
 *
 * https://leetcode.com/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (53.35%)
 * Likes:    14153
 * Dislikes: 855
 * Total Accepted:    1.3M
 * Total Submissions: 2.4M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * You are given an integer array height of length n. There are n vertical
 * lines drawn such that the two endpoints of the i^th line are (i, 0) and (i,
 * height[i]).
 *
 * Find two lines that together with the x-axis form a container, such that the
 * container contains the most water.
 *
 * Return the maximum amount of water a container can store.
 *
 * Notice that you may not slant the container.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array
 * [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the
 * container can contain is 49.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */

// @lc code=start
function maxArea(height: number[]): number {
  let i = 0,
    j = 0,
    left = 0,
    right = 0,
    max = 0
  let filtered = Array.from(height, (e, i) => i)

  const getArea = (i: number, j: number) => {
    return Math.min(height[i], height[j]) * (j - i)
  }

  while (filtered.length > 1) {
    const shifted = filtered.shift()
    if (shifted !== 0 && !shifted) return max
    i = shifted
    for (j = 0; j < filtered.length; j++) {
      const area = getArea(i, filtered[j])
      if (max < area) {
        max = area
        left = i
        right = filtered[j]
      }
    }
    const base = Math.min(height[left], height[right])
    filtered = filtered.filter((e) => height[e] > base)
  }
  return max
}
// @lc code=end
const height = [1, 8, 6, 2, 5, 4, 8, 3, 7]
console.log(height)
maxArea(height)
