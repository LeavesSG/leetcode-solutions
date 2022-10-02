/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */
use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let m = grid.len();
        let n = grid[0].len();
        fn get_siblings_2d(index: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
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
        let get_char = |index: (usize, usize)| grid[index.0][index.1];
        let filtered_adj = |index: (usize, usize)| {
            let result: Vec<_> = get_siblings_2d(index, (m, n))
                .into_iter()
                .filter(|adj| get_char(index) == get_char(*adj))
                .collect();
            result
        };
        let vertices: Vec<_> = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| (i, j)))
            .collect();

        fn cc_owned<T, P>(vertices: Vec<T>, get_adj: &P) -> Vec<(T, i32)>
        where
            T: Copy + Eq + std::hash::Hash,
            P: Fn(T) -> Vec<T>,
        {
            let mut checked = std::collections::HashMap::new();
            let mut index = 0;

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
        let mut island = std::collections::HashMap::new();
        cc.into_iter()
            .filter(|(index, _)| get_char(*index) == '1')
            .for_each(|(_, cci)| {
                island.insert(cci, true);
            });
        island.len() as i32
    }
}
// @lc code=end

#[test]
fn test() {
    use crate::vecnd;
    let grid = vecnd!(
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0']
    );
    let res = Solution::num_islands(grid);
    assert_eq!(res, 1);
}
