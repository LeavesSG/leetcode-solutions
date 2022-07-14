/*
 * @lc app=leetcode id=135 lang=rust
 *
 * [135] Candy
 */
use crate::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let m = ratings.len();
        let mut greedy = Vec::from([1]);

        fn max(i: i32, j: i32) -> i32 {
            if i > j {
                return i;
            };
            j
        }

        for i in 1..m {
            if ratings[i] > ratings[i - 1] {
                greedy.push(greedy[i - 1] + 1)
            } else {
                greedy.push(1)
            }
        }

        for i in (1..m).rev() {
            if ratings[i - 1] > ratings[i] {
                greedy[i - 1] = max(greedy[i - 1], greedy[i] + 1);
            }
        }

        let mut count = 0;
        for i in greedy {
            count += i;
        }
        count
    }
}
// @lc code=end

#[test]
pub fn test() {
    let ratings = Vec::from([1, 3, 2, 2, 1]);
    assert!(Solution::candy(ratings) == 7);
}
