pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut chars = s.chars();
        let mut stack: Vec<i32> = vec![single_roman_to_abs(chars.next().unwrap())];
        let mut res = 0;

        for current_roman in chars {
            let current_v = single_roman_to_abs(current_roman);
            let mut to_add = 0;

            while let Some(stacked_v) = stack.pop() {
                if stacked_v < current_v {
                    // substract all values in the stack because they preceed
                    // a larger roman number. For example, "IV" or "XXLIX"
                    // (even though the second case might not be a valid roman number).
                    to_add -= stacked_v;
                } else {
                    // Is this dead code?
                    // A: Apparently not. For example, "LXXX" requires us to push
                    // "L" back to the stack.
                    stack.push(stacked_v);
                    break;
                }
            }
            stack.push(current_v);
            res += to_add;
        }

        // sum all remaining stacked values
        while let Some(v) = stack.pop() {
            res += v;
        }
        res
    }
}

// pattern matching on char
fn single_roman_to_abs(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_49() {
        assert_eq!(Solution::roman_to_int(String::from("XLIX")), 49);
    }

    #[test]
    fn test_33() {
        assert_eq!(Solution::roman_to_int(String::from("XXXIII")), 33);
    }

    #[test]
    fn test_87() {
        assert_eq!(Solution::roman_to_int(String::from("LXXXVII")), 87);
    }

    #[test]
    fn test_39() {
        // even though this is not a valid roman
        assert_eq!(Solution::roman_to_int(String::from("XXLIX")), 39);
    }
}
