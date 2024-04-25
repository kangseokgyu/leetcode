use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pre_nums: HashMap<i32, usize> = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            let num = pre_nums.get(&(target - n));
            match num {
                Some(n) => return vec![*n as i32, i as i32],
                None => {
                    pre_nums.insert(n, i);
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
