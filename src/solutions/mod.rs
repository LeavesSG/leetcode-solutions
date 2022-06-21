pub struct Solution {}

impl Solution {
    pub fn test_all() -> Vec<Box<()>> {
        return Vec::from([
            Box::new(candy::test()),
            Box::new(max_profit::test()),
            Box::new(min_path_sum::test()),
            Box::new(set_zeroes::test()),
        ]);
    }
}

mod candy;
mod max_gap;
mod max_profit;
mod min_path_sum;
mod set_zeroes;
