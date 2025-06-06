// 130. Surrounded Regions
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        
        let m = board.len();
        let n = board[0].len();
        
        for j in 0..n {
            if board[0][j] == 'O' {
                Self::dfs(board, 0, j);
            }
            if board[m-1][j] == 'O' {
                Self::dfs(board, m-1, j);
            }
        }
        
        for i in 0..m {
            if board[i][0] == 'O' {
                Self::dfs(board, i, 0);
            }
            if board[i][n-1] == 'O' {
                Self::dfs(board, i, n-1);
            }
        }
        
        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    'O' => board[i][j] = 'X',
                    'S' => board[i][j] = 'O',
                    _ => {}
                }
            }
        }
    }
    
    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let m = board.len();
        let n = board[0].len();
        
        if i >= m || j >= n || board[i][j] != 'O' {
            return;
        }
        
        board[i][j] = 'S';
        
        if i > 0 {
            Self::dfs(board, i - 1, j);
        }
        if i + 1 < m {
            Self::dfs(board, i + 1, j);
        }
        if j > 0 {
            Self::dfs(board, i, j - 1);
        }
        if j + 1 < n {
            Self::dfs(board, i, j + 1);
        }
    }
}