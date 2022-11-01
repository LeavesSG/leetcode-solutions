/*
 * @lc app=leetcode id=715 lang=rust
 *
 * [715] Range Module
 */

// @lc code=start
struct RangeModule {
    vert: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        RangeModule { vert: vec![] }
    }

    fn add_range(&self, left: i32, right: i32) {
        let res_left = self.vert.binary_search(&left);

        let is_odd = |num| num % 2 == 1;

        let left_bound = match res_left {
            Ok(index) => {
                if is_odd(index) {
                    index
                } else {
                    index - 1
                }
            }
            Err(index) => {
                if is_odd(index) {
                    index - 1
                } else {
                    index
                }
            }
        };
    }

    fn query_range(&self, left: i32, right: i32) -> bool {}

    fn remove_range(&self, left: i32, right: i32) {}
}

// @lc code=end
