// 79. Word Search
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word_chars: Vec<char> = word.chars().collect();
        let m = board.len();
        let n = board[0].len();
        
        for i in 0..m {
            for j in 0..n {
                if Self::dfs(&mut board, &word_chars, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
    
    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], row: usize, col: usize, index: usize) -> bool {
        if index == word.len() {
            return true;
        }
        
        let m = board.len();
        let n = board[0].len();
        
        if row >= m || col >= n || board[row][col] != word[index] {
            return false;
        }
        
        let temp = board[row][col];
        board[row][col] = '#';
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            
            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                if Self::dfs(board, word, new_row, new_col, index + 1) {
                    board[row][col] = temp;
                    return true;
                }
            }
        }
        
        board[row][col] = temp;
        false
    }
}