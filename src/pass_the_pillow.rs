struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut position = 1;
        let mut moving_right = true;
        for i in 1..=time {
            if position == n {
                moving_right = false;
            } else if position == 1 {
                moving_right = true;
            }

            if moving_right {
                position += 1;
            } else {
                position -= 1;
            }
        }

        return position;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
        assert_eq!(Solution::pass_the_pillow(18, 38), 5);
    }
}
