/*
 * @lc app=leetcode id=41 lang=rust
 *
 * [41] First Missing Positive
 */

use super::Solution;
#[allow(dead_code)]

// @lc code=start

fn try_write(vec: &mut Vec<i32>, num: i32) -> () {
    if num < 1 || num > (vec.len()) as i32 {
        return;
    }
    let unum = (num - 1) as usize;
    let target = vec[unum];
    if num == target {
        return;
    } else {
        vec[unum] = num;
        try_write(vec, target)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            let val = nums[i];
            try_write(&mut nums, val);
        }
        let mut i = 0_i32;
        loop {
            if i as usize > nums.len() - 1 || nums[i as usize] != i + 1 {
                break;
            }
            i += 1;
        }
        i + 1
    }
}
// @lc code=end

#[test]

fn test() {
    let vec = vec![3, 1];
    let res = Solution::first_missing_positive(vec.clone());
    println!("{res}");
}
