/*
 * @lc app=leetcode id=164 lang=rust
 *
 * [164] Maximum Gap
 */
use crate::Solution;
#[allow(dead_code)]
// @lc code=start

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let m = nums.len();
        if m < 2 {
            return 0;
        }
        let mut lb = 0;
        let mut ub = 1;
        let mut min = 0;
        let mut max = 1;
        let mut i = 2;
        while i < max {
            if nums[i] < nums[lb] {
                if nums[lb] - nums[i] > nums[max] - nums[min] {
                    min = i;
                    max = lb;
                }
                lb = i;
            } else if nums[i] > nums[ub] {
                if nums[i] - nums[ub] > nums[max] - nums[min] {
                    max = i;
                    min = ub;
                }
                ub = i;
            }
            i -= 1;
        }

        nums[min] - nums[max]
    }
}

// @lc code=end
