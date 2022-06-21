mod solutions;
use solutions::Solution;

fn main() {
    let tests = Solution::test_all();
    for test in tests {
        drop(test);
    }
    println!("All test passed!");
}
