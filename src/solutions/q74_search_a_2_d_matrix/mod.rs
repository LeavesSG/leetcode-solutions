/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 */

use super::Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by(|row| {
            if (row[0]) > target {
                return Ordering::Greater;
            } else {
                match row.last().unwrap().cmp(&target) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => return Ordering::Equal,
                    Ordering::Greater => {
                        if let Ok(_) = row.binary_search(&target) {
                            return Ordering::Equal;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                }
            }
        }) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
// @lc code=end
