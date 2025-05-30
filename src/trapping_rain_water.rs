// 42. Trapping Rain Water
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }

        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];

        left_max[0] = height[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(height[i]);
        }

        right_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }

        let mut water_trapped = 0;
        for i in 0..n {
            water_trapped += (left_max[i].min(right_max[i]) - height[i]).max(0);
        }

        water_trapped
    }
}