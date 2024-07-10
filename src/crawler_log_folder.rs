struct Solution {}

impl Solution {
    #[allow(dead_code)]
    fn is_directory(s: &String) -> bool {
        *s != "../".to_string() && *s != "./".to_string()
    }

    #[allow(dead_code)]
    fn is_move_up(s: &String) -> bool {
        *s == "../".to_string()
    }

    #[allow(dead_code)]
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut deep = 0;

        for log in logs.iter() {
            dbg!(log);

            if Solution::is_directory(log) {
                deep += 1;
            }

            if Solution::is_move_up(log) && deep > 0 {
                deep -= 1;
            }
        }

        return deep;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(
            Solution::min_operations(
                vec![
                    "d1/".to_string(),
                    "d2/".to_string(),
                    "./".to_string(),
                    "d3/".to_string(),
                    "../".to_string(),
                    "d31/".to_string()
                ]
            ),
            3
        );
        assert_eq!(
            Solution::min_operations(
                vec!["./".to_string(), "ho3/".to_string(), "tl8/".to_string()]
            ),
            2
        );
    }
}
