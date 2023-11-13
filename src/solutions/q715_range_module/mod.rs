/*
 * @lc app=leetcode id=715 lang=rust
 *
 * [715] Range Module
 */

// @lc code=start
use std::{cmp::Ordering, fmt::Debug};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum RangeBound {
    Start(usize),
    End(usize),
}

impl RangeBound {
    fn get_index(&self) -> &usize {
        match self {
            Start(index) => index,
            End(index) => index,
        }
    }
    fn is_same_variant(&self, other: &Self) -> bool {
        match self {
            Start(_) => {
                matches!(other, Start(_))
            }
            End(_) => matches!(other, End(_)),
        }
    }
}
impl PartialOrd for RangeBound {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.get_index().cmp(other.get_index());
        Some(ord)
    }
}
impl Ord for RangeBound {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
use RangeBound::{End, Start};

#[derive(Debug)]
struct RangeModule {
    vertices: Vec<RangeBound>,
}
#[allow(dead_code)]
impl RangeModule {
    fn new() -> Self {
        RangeModule { vertices: vec![] }
    }
    fn search_pos(&self, target: &RangeBound) -> Result<usize, usize> {
        self.vertices.binary_search_by(|probe| probe.cmp(target))
    }
    fn insert_unchecked(&mut self, target: RangeBound) -> usize {
        let index = self.search_pos(&target);
        match index {
            Ok(index) => {
                self.vertices[index] = target;
                index
            }
            Err(index) => {
                self.vertices.insert(index, target);
                index
            }
        }
    }
    fn shift_index_checked(&self, index: usize, offset: i32) -> Option<usize> {
        let res = index as i32 + offset;
        match res >= 0 && res < self.vertices.len() as i32 {
            true => Some(res as usize),
            false => None,
        }
    }

    fn check_index_sibling(&self, index: usize) -> Option<(usize, usize)> {
        let left_sibling = self.shift_index_checked(index, -1);
        let right_sibling = self.shift_index_checked(index, 1);
        let mut res = vec![];
        let testing = [(left_sibling, Some(index)), (Some(index), right_sibling)];
        testing
            .iter()
            .for_each(|(left, right)| match (left, right) {
                (None, Some(right)) => {
                    if let End(_) = self.vertices[*right] {
                        res.push(right);
                    }
                }
                (Some(left), None) => {
                    if let Start(_) = self.vertices[*left] {
                        res.push(left);
                    }
                }
                (Some(left), Some(right)) => {
                    let range_bound = &self.vertices[*left];
                    let other = &self.vertices[*right];
                    if range_bound.is_same_variant(other) {
                        match self.vertices[index] {
                            Start(_) => res.push(right),
                            End(_) => res.push(left),
                        };
                    }
                }
                (_, _) => {}
            });
        if res.is_empty() {
            None
        } else {
            let min = *res.iter().min().unwrap();
            let max = res.into_iter().max().unwrap();
            Some((*min, max + 1))
        }
    }

    fn post_handle_change(&mut self, range: (usize, usize)) {
        let mut offset = 0;
        let vec = [range.0, range.1];
        vec.into_iter().for_each(|index| {
            let checked = self.check_index_sibling(index - offset);
            if let Some(range) = checked {
                self.vertices.splice(range.0..range.1, []);
                offset += range.1 - range.0;
            }
        });
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let start = self.insert_unchecked(Start(left as usize));
        let end = self.insert_unchecked(End(right as usize));
        self.post_handle_change((start, end));
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        let left = match self.vertices.binary_search_by(|probe| {
            match probe.get_index().cmp(&(left as usize)) {
                Ordering::Equal => Ordering::Less,
                some_ord => some_ord,
            }
        }) {
            Ok(index) => index,
            Err(index) => index,
        };
        let right = match self.vertices.binary_search_by(|probe| {
            match probe.get_index().cmp(&(right as usize)) {
                Ordering::Equal => Ordering::Greater,
                some_ord => some_ord,
            }
        }) {
            Ok(index) => index,
            Err(index) => index,
        };
        left == right && left > 0 && matches!(self.vertices[left - 1], Start(_))
    }
    fn remove_range(&mut self, left: i32, right: i32) {
        let start = self.insert_unchecked(End(left as usize));
        let end = self.insert_unchecked(Start(right as usize));
        self.post_handle_change((start, end));
    }
}
// @lc code=end

#[test]
fn range_module() {
    let mut range_mod = RangeModule::new();
    // let mut res;
    range_mod.add_range(6, 8);
    println!("{:?}", range_mod);
    range_mod.remove_range(7, 8);
    println!("{:?}", range_mod);
    range_mod.remove_range(8, 9);
    println!("{:?}", range_mod);
    // res = range_mod.query_range(50, 100);
    // println!("{}", res);
    // res = range_mod.query_range(180, 300);
    // println!("{}", res);
    // res = range_mod.query_range(600, 1000);
    // println!("{}", res);
    // range_mod.remove_range(50, 150);
    // println!("{:?}", range_mod);
    // res = range_mod.query_range(50, 100);
    // println!("{}", res);
}
