/*
 * @lc app=leetcode id=174 lang=rust
 *
 * [174] Dungeon Game
 */
use super::Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        fn recursion(
            i: usize,
            j: usize,
            m: usize,
            n: usize,
            hp: i32,
            dungeon: &Vec<Vec<i32>>,
            map: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            let min;
            let curr = dungeon[m - i][n - j];
            let before = (hp - curr).max(1);
            let find = map.get(&(m - i, n - j));
            println!("i:{},j:{},hp:{},curr:{},before:{}", i, j, hp, curr, before);
            match find {
                Some(v) => return *v,
                _ => match (i, j) {
                    (0, 0) => {
                        min = before;
                    }
                    (i, 0) => {
                        min = recursion(i - 1, j, m, n, before, &dungeon, map);
                    }
                    (0, j) => {
                        min = recursion(i, j - 1, m, n, before, &dungeon, map);
                    }
                    _ => {
                        let left = recursion(i - 1, j, m, n, before, &dungeon, map);
                        let right = recursion(i, j - 1, m, n, before, &dungeon, map);
                        min = left.min(right);
                    }
                },
            }
            map.insert((m - 1, n - 1), min);
            return min;
        }
        let mut map: HashMap<(usize, usize), i32> = HashMap::new();
        let result = recursion(0, 0, m - 1, n - 1, 1, &dungeon, &mut map);
        println!("{:?}", map);
        return result;
    }
}
// @lc code=end

#[test]
fn test() {
    let dungeun = vec![vec![0, -5], vec![0, 0]];
    let result = Solution::calculate_minimum_hp(dungeun);
    println!("{}", result);
}
