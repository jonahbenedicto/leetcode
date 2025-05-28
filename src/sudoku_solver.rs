// 37. Sudoku Solver
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
            for i in 0..9 {
                if board[row][i] == num || board[i][col] == num {
                    return false;
                }
            }
            let box_row = (row / 3) * 3;
            let box_col = (col / 3) * 3;
            for i in 0..3 {
                for j in 0..3 {
                    if board[box_row + i][box_col + j] == num {
                        return false;
                    }
                }
            }
            true
        }

        fn solve(board: &mut Vec<Vec<char>>) -> bool {
            for i in 0..9 {
                for j in 0..9 {
                    if board[i][j] == '.' {
                        for num in '1'..='9' {
                            if is_valid(board, i, j, num) {
                                board[i][j] = num;
                                if solve(board) {
                                    return true;
                                }
                                board[i][j] = '.';
                            }
                        }
                        return false;
                    }
                }
            }
            true
        }

        solve(board);
    }
}