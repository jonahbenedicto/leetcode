// 73. Set Matrix Zeroes
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        
        let mut first_row_zero = false;
        let mut first_col_zero = false;
        
        for j in 0..n {
            if matrix[0][j] == 0 {
                first_row_zero = true;
                break;
            }
        }
        
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col_zero = true;
                break;
            }
        }
        
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0; 
                    matrix[0][j] = 0; 
                }
            }
        }
        
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        
        if first_row_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
        
        if first_col_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}