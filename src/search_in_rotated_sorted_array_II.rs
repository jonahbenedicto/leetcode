// 81. Search in Rotated Sorted Array II
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid] == target {
                return true;
            }
            
            if nums[left] == nums[mid] && nums[mid] == nums[right] {
                left += 1;
                if right > 0 {
                    right -= 1;
                }
            }

            else if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            else {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    if mid > 0 {
                        right = mid - 1;
                    } else {
                        break;
                    }
                }
            }
        }
        
        false
    }
}