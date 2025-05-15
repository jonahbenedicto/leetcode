mod two_sum;
use two_sum::Solution;

fn main() {
    let nums = vec![2,7,11,15];
    let result = Solution::two_sum(nums, 9);
    println!("{:?}", result); 
}
