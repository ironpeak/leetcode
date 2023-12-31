pub struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut instances: [[Option<i32>; 2]; 26] = [[None; 2]; 26];
        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - 'a' as u8) as usize;
            if instances[index][0].is_none() {
                instances[index][0] = Some(i as i32);
            } else {
                instances[index][1] = Some(i as i32);
            }
        }
        let mut output = -1;
        for i in 0..26 {
            if let (Some(left), Some(right)) = (instances[i][0], instances[i][1]) {
                if right - left - 1 > output {
                    output = right - left - 1;
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".to_string()),
            0
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }
}
