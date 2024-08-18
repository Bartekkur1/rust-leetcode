#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut sum = num_bottles;
        let mut empty_bottles = num_bottles;

        loop {
            let exchanged_bottles = empty_bottles / num_exchange;
            sum += exchanged_bottles;
            let left_over = empty_bottles - exchanged_bottles * num_exchange;
            empty_bottles = exchanged_bottles + left_over;

            if empty_bottles < num_exchange {
                break;
            }
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
        assert_eq!(Solution::num_water_bottles(17, 3), 25);
    }
}
