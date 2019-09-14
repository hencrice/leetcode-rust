pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s == "" {
            true
        } else {
            // 1 byte per ASCII char
            let mut stack = Vec::with_capacity(s.len());
            for c in s.chars() {
                match c {
                    '(' | '[' | '{' => stack.push(c),
                    ')' => {
                        if !pop_and_compare(&mut stack, '(') {
                            return false;
                        }
                    }
                    ']' => {
                        if !pop_and_compare(&mut stack, '[') {
                            return false;
                        }
                    }
                    '}' => {
                        if !pop_and_compare(&mut stack, '{') {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            // made a mistake here, thought I can directly return true
            stack.is_empty()
        }
    }
}

fn pop_and_compare(stack: &mut Vec<char>, closing: char) -> bool {
    match stack.pop() {
        None => false,
        Some(s) => s == closing,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_empty() {
        assert_eq!(Solution::is_valid(String::from("")), true);
    }

    #[test]
    fn test_valid() {
        assert_eq!(Solution::is_valid(String::from("({[([{[[]]}])]})")), true);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(Solution::is_valid(String::from("{{]]")), false);
    }

    #[test]
    fn test_invalid_no_closing() {
        assert_eq!(Solution::is_valid(String::from("{")), false);
    }
}
