struct Solution {}

impl Solution {
    #[allow(dead_code)]
    fn roman_cache(c: char) -> i32 {
        return match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!(),
        };
    }

    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let roman_nums_arr: Vec<i32> = s
            .chars()
            .map(|x| Solution::roman_cache(x))
            .collect();

        let mut sum = 0;
        for window in roman_nums_arr.windows(2) {
            let (value, next_value) = (window[0], window[1]);
            if next_value > value {
                sum -= value;
            } else {
                sum += value;
            }
        }

        if let Some(&last) = roman_nums_arr.last() {
            sum += last;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("VI")), 6);
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }
}
