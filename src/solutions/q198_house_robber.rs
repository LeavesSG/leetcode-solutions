/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
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
            i2
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
        dp[m - 1]
    }
}

// @lc code=end
#[test]
pub fn test() {
    let a = [2, 7, 9, 3, 1].to_vec();
    let b = Solution::rob(a);
    println!("{}", b);
}
