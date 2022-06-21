/*
 * @lc app=leetcode id=55 lang=typescript
 *
 * [55] Jump Game
 *
 * https://leetcode.com/problems/jump-game/description/
 *
 * algorithms
 * Medium (37.47%)
 * Likes:    9941
 * Dislikes: 578
 * Total Accepted:    915.3K
 * Total Submissions: 2.4M
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * You are given an integer array nums. You are initially positioned at the
 * array's first index, and each element in the array represents your maximum
 * jump length at that position.
 *
 * Return true if you can reach the last index, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last
 * index.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum
 * jump length is 0, which makes it impossible to reach the last index.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * 0 <= nums[i] <= 10^5
 *
 *
 */
// @lc code=start
function canJump(nums) {
    var ub = 0, p;
    while (ub <= nums.length) {
        p = ub;
        while (p >= 0 && nums[p] + p <= ub)
            p--;
        if (nums[p] + p > ub) {
            ub = nums[p] + p;
        }
        else {
            break;
        }
    }
    return ub >= nums.length - 1;
}
// @lc code=end
var t55 = [2, 3, 1, 1, 4];
console.log(canJump(t55));
