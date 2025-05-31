// 63. Unique Paths II
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        
        let mut dp = vec![vec![0_i64; n]; m];
        
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        dp[0][0] = 1;
        
        for i in 1..m {
            if obstacle_grid[i][0] == 0 && dp[i-1][0] > 0 {
                dp[i][0] = 1;
            }
        }
        
        for j in 1..n {
            if obstacle_grid[0][j] == 0 && dp[0][j-1] > 0 {
                dp[0][j] = 1;
            }
        }
        
        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 0 {
                    dp[i][j] = dp[i-1][j] + dp[i][j-1];
                }
            }
        }
        
        dp[m-1][n-1] as i32
    }
}