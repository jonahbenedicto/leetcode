// 51. N-Queens
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut board = vec![vec!['.'; n]; n];
        
        Self::backtrack(0, n, &mut board, &mut result);
        
        result
    }
    
    fn backtrack(row: usize, n: usize, board: &mut Vec<Vec<char>>, result: &mut Vec<Vec<String>>) {
        if row == n {
            let solution = board.iter()
                .map(|row| row.iter().collect::<String>())
                .collect();
            result.push(solution);
            return;
        }
        
        for col in 0..n {
            if Self::is_valid(row, col, n, board) {
                board[row][col] = 'Q';
                
                Self::backtrack(row + 1, n, board, result);
                
                board[row][col] = '.';
            }
        }
    }
    
    fn is_valid(row: usize, col: usize, n: usize, board: &Vec<Vec<char>>) -> bool {
        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        
        let mut i = row as isize - 1;
        let mut j = col as isize - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        
        let mut i = row as isize - 1;
        let mut j = col as isize + 1;
        while i >= 0 && j < n as isize {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        
        true
    }
}