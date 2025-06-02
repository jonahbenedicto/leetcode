// 74. Search a 2D Matrix
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n - 1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            let row = mid / n;
            let col = mid % n;
            let mid_value = matrix[row][col];
            
            if mid_value == target {
                return true;
            } else if mid_value < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }
        
        false
    }
}