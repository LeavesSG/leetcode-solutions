/*
 * @lc app=leetcode id=319 lang=rust
 *
 * [319] Bulb Switcher
 */

use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let mut pre = vec![1; n as usize];
        for i in 1..n {
            for j in (i..n).step_by(i as usize + 1) {
                pre[j as usize] = match pre[j as usize] {
                    0 => 1,
                    _ => 0,
                }
            }
        }
        let res = pre.into_iter().reduce(|pre, next| pre + next);
        res.unwrap_or(0)
    }
}
// @lc code=end

#[test]
fn test() {
    let res = Solution::bulb_switch(9999999);
    println!("{res}");
}
