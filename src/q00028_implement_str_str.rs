pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            0
        } else {
            let v: Vec<_> = haystack.match_indices(&needle).collect();
            if v.is_empty() {
                -1
            } else {
                let (res, _) = v[0];
                res as i32
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ll_in_hello() {
        assert_eq!(Solution::str_str(String::from("hello"), String::from("ll")), 2);
    }

    #[test]
    fn test_empty_needle() {
        assert_eq!(Solution::str_str(String::from("hello"), String::from("")), 0);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(Solution::str_str(String::from("aaaaa"), String::from("ll")), -1);
    } 

    #[test]
    fn test_empty_haystack() {
        assert_eq!(Solution::str_str(String::from(""), String::from("ll")), -1);
    } 
}
