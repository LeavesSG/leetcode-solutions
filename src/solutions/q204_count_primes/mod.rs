/*
 * @lc app=leetcode id=204 lang=rust
 *
 * [204] Count Primes
 */

use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut primes: Vec<_> = (1..n + 1).into_iter().map(|_| true).collect();
        primes[0] = false;
        primes[1] = false;
        let mut ptr = 2;
        while ptr < n - 1 {
            for ptr_inner in (ptr..n).step_by(ptr as usize).skip(1) {
                primes[ptr_inner as usize] = false;
            }
            ptr += 1;
            while !primes[ptr as usize] && ptr < n - 1 {
                ptr += 1
            }
        }

        let mut count = 0;
        for bool in &primes {
            if *bool {
                count += 1
            }
        }

        count
    }
}
// @lc code=end

#[test]
fn test() {
    let result = Solution::count_primes(7);
    println!("{}", result)
}
