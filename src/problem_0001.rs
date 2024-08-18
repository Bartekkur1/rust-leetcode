use std::{ collections::HashMap, vec };

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // value: position
        let mut cache: HashMap<i32, usize> = HashMap::new();

        for (index, &item) in nums.iter().enumerate() {
            let diff = target - item;
            let cache_record = cache.get(&diff);
            if cache_record.is_some() {
                let result = vec![*cache_record.unwrap() as i32, index as i32];
                return result;
            } else {
                cache.insert(item, index);
            }
        }

        panic!();
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0001::Solution;

    #[test]
    fn test_solve_two_sum_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_solve_two_sum_2() {
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
    }

    #[test]
    fn test_solve_two_sum_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
