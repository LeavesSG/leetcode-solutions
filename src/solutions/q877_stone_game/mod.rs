/*
 * @lc app=leetcode id=877 lang=rust
 *
 * [877] Stone Game
 */
use super::Solution;

// @lc code=start

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut hashmap: HashMap<usize, i32> = HashMap::new();
        let sum: i32 = piles.iter().sum();
        fn max_piles(
            start: usize,
            end: usize,
            source: &Vec<i32>,
            hashmap: &mut HashMap<usize, i32>,
            len: usize,
            sum: i32,
        ) -> i32 {
            let key = len * start + end;
            let found = hashmap.get(&key);
            match found {
                Some(found) => *found,
                _ => {
                    let larger;
                    if end - start <= 2 {
                        if source[start] > source[start + 1] {
                            larger = source[start];
                        } else {
                            larger = source[start + 1];
                        }
                    } else {
                        let left = sum
                            - max_piles(start + 1, end, source, hashmap, len, sum - source[start]);
                        let right = sum
                            - max_piles(
                                start,
                                end - 1,
                                source,
                                hashmap,
                                len,
                                sum - source[end - 1],
                            );
                        if left > right {
                            larger = left;
                        } else {
                            larger = right
                        }
                    }
                    hashmap.insert(key, larger);
                    larger
                }
            }
        }
        let len = piles.len();
        let max = max_piles(0, len, &piles, &mut hashmap, len, sum);
        max * 2 > sum
    }
}
// @lc code=end

#[test]
fn test() {
    let a = [1, 2, 3, 4, 5, 7].to_vec();
    let result = Solution::stone_game(a);
    println!("{}", result);
}
