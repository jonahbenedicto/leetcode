// Q1. Partition Array into Two Equal Product Subsets - Weekly Contest 452
pub struct Solution;
impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        let n = nums.len();

        let mut total = 1 as i64;
        for &num in &nums {
            total *= num as i64;
        }

        if total != target * target {
            return false;
        }

        for &num in &nums {
            if num as i64 == target {
                return true;
            }
        }

        for k in 1..=(n/2) {
            let mut indices: Vec<usize> = (0..k).collect();
            
            loop {
                let mut product = 1 as i64;
                for &idx in &indices {
                    product *= nums[idx] as i64;
                }
                
                if product == target {
                    return true;
                }
                
                let mut i = k - 1;
                while i > 0 && indices[i] == n - k + i {
                    i -= 1;
                }
                
                if i == 0 && indices[i] == n - k {
                    break;
                }
                
                indices[i] += 1;
                for j in i+1..k {
                    indices[j] = indices[j-1] + 1;
                }
            }
        }
        
        false
    }
}