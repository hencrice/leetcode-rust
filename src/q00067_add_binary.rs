use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = false;
        let mut res = vec![];

        // pad '0's to the shorter string so that a and b 
        // are of the same length. This makes it easier to sum
        let (iter1, extended_chars) = if a.len() > b.len() {
            let mut extended = vec!['0'; a.len()-b.len()];
            extended.extend(b.chars());
            (a.chars().rev(), extended)
        } else {
            let mut extended = vec!['0'; b.len() - a.len()];
            extended.extend(a.chars());
            (b.chars().rev(), extended)
        };

        for (b1, b2) in iter1.zip(extended_chars.iter().rev()) {
            match (b1, b2) {
                ('0', '0') => {
                    if carry {
                        res.push('1');
                    } else {
                        res.push('0')
                    }
                    carry = false;
                },
                ('0', '1') | ('1', '0') =>{
                    if carry {
                        res.push('0');
                        carry = true;
                    } else {
                        res.push('1');
                        carry = false;
                    }
                },
                ('1', '1') => {
                    if carry {
                        res.push('1');
                    } else {
                        res.push('0')
                    }
                    carry = true;  
                },
                _ => panic!("unexpected match case"),
            }
        }

        if carry {
            // carry happens on the most significant bit
            res.push('1');
        }

        String::from_iter(res.iter().rev())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::add_binary(String::from("100"), String::from("1")), "101");
    }

    #[test]
    fn test_carry_on_most_significant_digit() {
        assert_eq!(Solution::add_binary(String::from("111"), String::from("1")), "1000");
    }

    #[test]
    fn test_0() {
        assert_eq!(Solution::add_binary(String::from("0"), String::from("1")), "1");
    }

    #[test]
    fn test_slightly_more_involved() {
        assert_eq!(Solution::add_binary(String::from("1010"), String::from("1011")), "10101");
    }
}
