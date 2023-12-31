pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complement = std::collections::HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = complement.get(&(target - num)) {
                return vec![*j as i32, i as i32];
            }
            complement.insert(num, i);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
