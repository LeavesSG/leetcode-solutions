/*
 * @lc app=leetcode id=419 lang=rust
 *
 * [419] Battleships in a Board
 */

use super::Solution;
#[allow(dead_code)]
// @lc code=start

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        if board.is_empty() {
            return 0;
        }
        let m = board.len();
        let n = board[0].len();

        fn get_siblings_2d(index: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
            // let validate = |i: i32, j: i32, m: usize, n: usize| {
            //     i >= 0 && j >= 0 && i < m as i32 && j < n as i32
            // };
            // let i = index.0 as i32;
            // let j = index.1 as i32;
            // let (m, n) = size;
            // [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)]
            //     .into_iter()
            //     .filter(|(i, j)| validate(*i, *j, m, n))
            //     .map(|(i, j)| (i as usize, j as usize))
            //     .collect()
            let (i, j) = index;
            let mut adj = vec![];
            if i > 0 {
                adj.push((i - 1, j));
            }
            if i < size.0 - 1 {
                adj.push((i + 1, j));
            }
            if j > 0 {
                adj.push((i, j - 1));
            }
            if j < size.1 - 1 {
                adj.push((i, j + 1));
            }
            adj
        }

        let get_char = |index: (usize, usize)| board[index.0][index.1];
        let filtered_adj = |index: (usize, usize)| {
            let result: Vec<_> = get_siblings_2d(index, (m, n))
                .into_iter()
                .filter(|adj| get_char(index) == get_char(*adj))
                .collect();
            result
        };

        let vertices: Vec<_> = board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| (i, j)))
            .collect();

        fn bfs_owned<T, P>(start: T, get_adj: &P) -> Vec<T>
        where
            T: Copy + Eq + std::hash::Hash,
            P: Fn(T) -> Vec<T>,
        {
            let mut queue = vec![start];
            let mut checked = std::collections::HashMap::new();
            while !queue.is_empty() {
                match queue.pop() {
                    Some(item) => {
                        checked.insert(item, true);
                        get_adj(item)
                            .into_iter()
                            .for_each(|adj| match checked.get(&adj) {
                                None => {
                                    queue.push(adj);
                                }
                                Some(_) => (),
                            });
                    }
                    None => (),
                }
            }
            checked.into_iter().map(|(key, _)| key).collect()
        }

        fn cc_owned<T, P>(vertices: Vec<T>, get_adj: &P) -> Vec<(T, i32)>
        where
            T: Copy + Eq + std::hash::Hash,
            P: Fn(T) -> Vec<T>,
        {
            let mut checked = std::collections::HashMap::new();
            let mut index = 0;
            vertices
                .into_iter()
                .for_each(|vertex| match checked.get(&vertex) {
                    None => {
                        let b = bfs_owned(vertex, &get_adj);
                        b.into_iter().for_each(|item| {
                            checked.insert(item, index);
                        });
                        index += 1;
                    }
                    Some(_) => (),
                });
            checked.into_iter().collect()
        }

        let cc = cc_owned(vertices, &filtered_adj);
        let mut ship = std::collections::HashMap::new();
        cc.into_iter()
            .for_each(|(key, value)| match ship.get(&value) {
                Some(_) => (),
                None => {
                    if get_char(key) == 'X' {
                        ship.insert(value, true);
                    } else {
                        ship.insert(value, false);
                    }
                }
            });
        ship.into_iter().filter(|(_, val)| *val).count() as i32
    }
}
// @lc code=end

#[test]
fn test() {
    use crate::vecnd;
    let board = vecnd!(['X', '.'], ['.', 'X']);
    let result = Solution::count_battleships(board);
    assert_eq!(result, 2);
}
