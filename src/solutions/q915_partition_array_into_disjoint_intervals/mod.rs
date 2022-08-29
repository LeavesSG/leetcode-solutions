/*
 * @lc app=leetcode id=915 lang=rust
 *
 * [915] Partition Array in&to Disjoint Intervals
 */
use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let lo = nums[0];
        let hi = nums[n - 1];

        let mut max: Vec<i32> = Vec::new();
        let mut min: Vec<i32> = Vec::new();
        max.push(lo);
        min.push(hi);

        for i in 1..n {
            max.push(nums[i].max(max[i - 1]));
        }

        for i in 1..n {
            min.push(nums[n - i - 1].min(min[i - 1]));
        }
        min.reverse();
        // println!("{:?}\n{:?}", max, min);
        let mut part: i32 = 0;
        while (max[part as usize] > min[part as usize + 1]) && ((part as usize) < n - 1) {
            part += 1
        }

        part + 1
    }
}
// @lc code=end

#[test]
fn test() {
    let nums = [1, 1].to_vec();
    let test = Solution::partition_disjoint(nums);
    println!("{}", test)
}
