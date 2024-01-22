/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

use super::Solution;

// @lc code=start
struct MonoStack<T>
where
    T: Ord,
{
    ord: std::cmp::Ordering,
    buf: Vec<T>,
}

impl<T: Ord> MonoStack<T> {
    fn push(&mut self, item: T) -> impl Iterator<Item = T> + '_ {
        let mut ptr = self.buf.len();
        if ptr > 0 {
            while ptr > 0 && item.cmp(&self.buf[ptr - 1]) == self.ord {
                ptr -= 1;
            }
        }
        self.buf.splice(ptr.., [item])
    }
}

#[derive(Debug)]
struct Enumerable<T: Ord>(usize, T);

impl PartialEq for Enumerable<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for Enumerable<i32> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Eq for Enumerable<i32> {}

impl Ord for Enumerable<i32> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl Solution {
    #[allow(dead_code, unused_must_use)]
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        let len = heights.len();
        let mut areas = vec![0; len];
        let mut stack: MonoStack<Enumerable<i32>> = MonoStack {
            ord: std::cmp::Ordering::Less,
            buf: Vec::with_capacity(len + 1),
        };
        let mut pop;
        for rb in 0..len {
            pop = stack.push(Enumerable(rb, heights[rb])).collect::<Vec<_>>();
            let mut lb = if stack.buf.len() > 1 {
                stack.buf[stack.buf.len() - 2].0 as i32
            } else {
                stack.buf.len() as i32 - 2
            };

            pop.into_iter().for_each(|Enumerable(j, jh)| {
                let len = rb as i32 - lb - 1;
                areas[j] = len * jh;
                lb = j as i32;
            });
        }
        areas.into_iter().max().unwrap_or(0)
    }
}
// @lc code=end

#[test]
fn test() {
    let res = Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0]);
    // println!("{}", res);
}

// [3, 6, 5, 7, 4, 8, 1, 0]
// [], []
// [3] ,[]
// [3, 6], []
// [3, 5], [6]
// [3, 5, 7], []
// [3, 4], [5, 7]
// [3, 4, 8], []
// [1], [3, 4, 8]
// [0], [1]
