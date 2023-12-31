pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Sort
        // We can sort the array to make it easier and faster to find the triplets.
        nums.sort();

        // Dedup
        // We can remove some duplicate values since the solution set can not contain duplicate triplets.
        // * 3x = 0 => x = 0 so we can remove all but the first 3 zeros.
        // * For all other values we just need to keep the first 2 values.
        // NOTE: there maybe some room for optimization because of constraint: -10^5 <= nums[i] <= 10^5
        let mut count = 0;
        let mut last_num = None;

        nums.retain(|&num| {
            if last_num == Some(num) {
                count += 1;
            } else {
                count = 1;
            }

            let should_remove = (num != 0 && count > 2) || (num == 0 && count > 3);

            last_num = Some(num);

            !should_remove
        });

        // Find triplets
        // Iterate through the array and for each element nums[i] we try to find two other elements.
        // We can use two pointers left and right to find the other two elements since the array is sorted.
        // We start with left = i + 1 and right = n - 1 and we move the pointers depending on the sum of the three elements.
        let mut triplets = Vec::with_capacity(nums.len());
        let n = nums.len();
        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, n - 1);

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum != 0 {
                    if sum > 0 {
                        right -= 1;
                    } else {
                        left += 1;
                    }
                } else {
                    triplets.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }

        return triplets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 1, 1, 2]),
            vec![vec![-2, 0, 2], vec![-2, 1, 1]]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
    }
}
