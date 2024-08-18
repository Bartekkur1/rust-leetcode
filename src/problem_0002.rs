// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    fn from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &value in nums.iter().rev() {
            let mut new_node = Box::new(ListNode::new(value));
            new_node.next = head;
            head = Some(new_node);
        }

        head
    }

    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut result_vec: Vec<i32> = Vec::new();
        let mut current1 = &l1;
        let mut current2 = &l2;
        let mut carry = 0;

        while current1.is_some() && current2.is_some() {
            if let (Some(node1), Some(node2)) = (current1.as_ref(), current2.as_ref()) {
                let mut sum = node1.val + node2.val + carry;
                carry = 0;
                if sum >= 10 {
                    carry = 1;
                    sum -= 10;
                }
                result_vec.push(sum);
                current1 = &node1.next;
                current2 = &node2.next;
            }
        }

        while let Some(node1) = current1 {
            let mut sum = node1.val + carry;
            carry = 0;
            if sum >= 10 {
                carry = 1;
                sum -= 10;
            }
            result_vec.push(sum);
            current1 = &node1.next;
        }

        while let Some(node2) = current2 {
            let mut sum = node2.val + carry;
            carry = 0;
            if sum >= 10 {
                carry = 1;
                sum -= 10;
            }
            result_vec.push(sum);
            current2 = &node2.next;
        }

        if carry > 0 {
            result_vec.push(carry);
        }

        return Solution::from_vec(result_vec);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve_two_sum_1() {
        let l1 = Solution::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = Solution::from_vec(vec![9, 9, 9, 9]);
        // let l3 = Solution::from_vec(vec![8, 0, 7]);

        let result = Solution::add_two_numbers(l1, l2);
        let expected_result = Solution::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(result, expected_result);
    }
}
