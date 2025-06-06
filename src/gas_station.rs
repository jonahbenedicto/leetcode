// 134. Gas Station
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_tank = 0;
        let mut curr_tank = 0;
        let mut starting_station = 0;
        
        for i in 0..n {
            total_tank += gas[i] - cost[i];
            curr_tank += gas[i] - cost[i];
            
            if curr_tank < 0 {
                starting_station = i + 1;
                curr_tank = 0;
            }
        }
        
        if total_tank < 0 {
            -1
        } else {
            starting_station as i32
        }
    }
}