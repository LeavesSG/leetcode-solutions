/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 *
 * https://leetcode.com/problems/house-robber/description/
 *
 * algorithms
 * Medium (46.54%)
 * Likes:    12799
 * Dislikes: 275
 * Total Accepted:    1.2M
 * Total Submissions: 2.5M
 * Testcase Example:  '[1,2,3,1]'
 *
 * You are a professional robber planning to rob houses along a street. Each
 * house has a certain amount of money stashed, the only constraint stopping
 * you from robbing each of them is that adjacent houses have security systems
 * connected and it will automatically contact the police if two adjacent
 * houses were broken into on the same night.
 *
 * Given an integer array nums representing the amount of money of each house,
 * return the maximum amount of money you can rob tonight without alerting the
 * police.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house
 * 5 (money = 1).
 * Total amount you can rob = 2 + 9 + 1 = 12.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 400
 *
 *
 */
use crate::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let m = nums.len();
        let mut dp: Vec<i32> = Vec::from([]);

        fn larger(i1: i32, i2: i32) -> i32 {
            if i1 > i2 {
                return i1;
            }
            return i2;
        }
        if m == 0 {
            return 0;
        }
        dp.push(nums[0]);
        if m == 1 {
            return dp[0];
        }
        dp.push(larger(nums[0], nums[1]));
        for i in 2..m {
            dp.push(larger(dp[i - 1], dp[i - 2] + nums[i]))
        }
        return dp[m - 1];
    }
}

// @lc code=end
#[test]
pub fn test() {
    let a = [2, 7, 9, 3, 1].to_vec();
    let b = Solution::rob(a);
    println!("{}", b);
}
