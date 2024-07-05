struct Solution {}

impl Solution {
    fn get_nth_char(s: &str, index: usize) -> Option<char> {
        s.chars().nth(index)
    }

    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x_str = x.to_string();

        let mut left_index = 0;
        let mut right_index = x_str.chars().count() - 1;

        loop {
            if left_index >= right_index {
                break;
            }
            if
                Solution::get_nth_char(&x_str, left_index) !=
                Solution::get_nth_char(&x_str, right_index)
            {
                return false;
            }
            left_index += 1;
            right_index -= 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_palindrome_number() {
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(123321), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(123421), false);
    }
}
