struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut players: Vec<i32> = (1..=n).collect();
        let mut cursor: i32 = 0;

        while players.len() > 1 {
            cursor = (cursor + k - 1) % (players.len() as i32);
            players.remove(cursor as usize);
        }

        return *players.get(0).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
