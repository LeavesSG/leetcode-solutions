use std::usize;

/*
 * @lc app=leetcode id=174 lang=rust
 *
 * [174] Dungeon Game
 */
use super::Solution;
// @lc code=start

#[allow(dead_code)]
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon.first().unwrap().len();
        let mut min_health_at = vec![0; m * n];
        min_health_at.fill(0);
        let rc_to_index = |r: usize, c: usize| r * n + c;
        min_health_at[rc_to_index(m - 1, n - 1)] = 1;
        let mut q = std::collections::VecDeque::from(vec![(m - 1, n - 1)]);
        while let Some((r, c)) = q.pop_front() {
            let room = dungeon[r][c];
            let prerequisite = min_health_at[rc_to_index(r, c)];
            let required = std::cmp::max(1, prerequisite - room);

            [(r.checked_sub(1), Some(c)), (Some(r), c.checked_sub(1))]
                .into_iter()
                .for_each(|(r, c)| {
                    if let (Some(r), Some(c)) = (r, c) {
                        let idx = rc_to_index(r, c);
                        let req = min_health_at[idx];
                        if req != 0 {
                            if required < req {
                                min_health_at[idx] = std::cmp::min(req, required);
                                q.push_back((r, c));
                            }
                        } else {
                            min_health_at[idx] = required;
                            q.push_back((r, c));
                        }
                    }
                });
        }
        std::cmp::max(1, min_health_at[0] - dungeon[0][0])
    }
}
// @lc code=end

#[test]
fn test() {
    use crate::vecnd;
    let dungeon = vecnd![[2], [1]];
    let result = Solution::calculate_minimum_hp(dungeon);
    println!("{}", result);
}
