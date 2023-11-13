/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 */

use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = mat
            .into_iter()
            .map(|item| {
                item.into_iter()
                    .map(|item| match item {
                        1 => -1,
                        _ => 0,
                    })
                    .collect()
            })
            .collect();
        fn get_adj(i: usize, j: usize, result: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
            let row_max = &result.len();
            let col_max = &result[0].len();

            let mut adj = vec![];
            // top
            if i > 0 {
                adj.push((i - 1, j));
            }
            // bottom
            if i < row_max - 1 {
                adj.push((i + 1, j));
            }
            // left
            if j > 0 {
                adj.push((i, j - 1));
            }
            // right
            if j < col_max - 1 {
                adj.push((i, j + 1));
            }
            adj
        }
        fn get_min(i: usize, j: usize, vec2d: &Vec<Vec<i32>>) -> i32 {
            get_adj(i, j, vec2d)
                .into_iter()
                .map(|(row, col)| vec2d[row][col])
                .filter(|x| *x >= 0)
                .min()
                .unwrap_or(-1)
        }

        // BFS queue
        let mut que = vec![];
        let mut buf = vec![];
        let mut dis = 1;
        let mut checked = std::collections::HashMap::new();
        // init queue items
        for (i, row) in result.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if get_min(i, j, &result) == 0 && *item != 0 {
                    que.push((i, j));
                    checked.insert((i, j), true);
                }
            }
        }

        loop {
            for (row, col) in que {
                result[row][col] = dis;
                let map = &mut checked;
                let adj = get_adj(row, col, &result);

                let trim = adj.into_iter().filter(|(row, col)| {
                    let checked = map.get(&(*row, *col)).is_none();
                    map.insert((*row, *col), true);
                    result[*row][*col] < 0 && checked
                });

                trim.for_each(|item| buf.push(item));
            }

            // next
            que = buf;
            buf = vec![];
            dis += 1;

            if que.is_empty() {
                break;
            }
        }
        result
    }
}
// @lc code=end

#[test]
fn test() {
    use crate::vecnd;

    let mat = vecnd![
        [1, 1, 0, 0, 1, 0, 0, 1, 1, 0],
        [1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 0, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
        [0, 1, 0, 1, 1, 0, 1, 1, 1, 1],
        [1, 1, 1, 0, 1, 0, 1, 1, 1, 1],
    ];
    let res = Solution::update_matrix(mat);
    let exp = vecnd![
        [2, 1, 0, 0, 1, 0, 0, 1, 1, 0],
        [1, 0, 0, 1, 0, 1, 1, 2, 2, 1],
        [1, 1, 1, 0, 0, 1, 2, 2, 1, 0],
        [0, 1, 2, 1, 0, 1, 2, 3, 2, 1],
        [0, 0, 1, 2, 1, 2, 1, 2, 1, 0],
        [1, 1, 2, 3, 2, 1, 0, 1, 1, 1],
        [0, 1, 2, 3, 2, 1, 1, 0, 0, 1],
        [1, 2, 1, 2, 1, 0, 0, 1, 1, 2],
        [0, 1, 0, 1, 1, 0, 1, 2, 2, 3],
        [1, 2, 1, 0, 1, 0, 1, 2, 3, 4]
    ];
    assert_eq!(exp, res);
}
