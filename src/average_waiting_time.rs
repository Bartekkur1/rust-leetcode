struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut time_end = 0;
        #[warn(unused_assignments)]
        let mut start_time = 0;
        let mut waiting: Vec<i64> = Vec::new();

        for customer in customers.into_iter() {
            let arrival: i64 = customer[0] as i64;
            let time: i64 = customer[1] as i64;

            if arrival > time_end {
                start_time = arrival;
            } else {
                start_time = time_end;
            }

            time_end = start_time + time;
            waiting.push(time_end - arrival);
        }

        return waiting.iter().fold(0f64, |sum, i| sum + (*i as f64)) / (waiting.len() as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve() {
        assert_eq!(Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]), 5.0);
        assert_eq!(
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25
        );
    }
}
