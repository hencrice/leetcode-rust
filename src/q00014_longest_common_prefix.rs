use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = vec![];

        // create list of iterators for each word in strs, then loop over each and call next() once.
        // Since we would like to modify individual iterators by calling its `next()`, we need to
        // delcare this vector as mutable.
        let mut iterators_vec: Vec<std::str::Chars> = strs.iter().map(|x| x.chars()).collect();

        // the problem states that "All given inputs are in lowercase letters a-z." so
        // it's ok to use '0' as the initial value.
        let mut last_char_in_common_prefix = '0';
        let mut index = 0;

        // loop over a vector of iterators until one of it returns None
        while let Some(it) = iterators_vec.get_mut(index) {
            if index == 0 {
                // `it` is the iterator of the first word in `strs`
                match it.next() {
                    Some(c) => {
                        last_char_in_common_prefix = c;

                        // with input ["a"], it's considered to have a common prefix "a"
                        if index == strs.len() - 1 {
                            res.push(c)
                        }
                    }
                    None => break,
                }
            } else if index == strs.len() - 1 {
                // `it` is the iterator of the last word in `strs`
                match it.next() {
                    Some(c) if c == last_char_in_common_prefix => res.push(c),
                    _ => break,
                }
            } else {
                match it.next() {
                    Some(c) if c == last_char_in_common_prefix => last_char_in_common_prefix = c,
                    _ => break,
                }
            }

            index += 1;
            index %= strs.len();
        }

        String::from_iter(res.iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_str_vec(strs: Vec<&str>) -> Vec<String> {
        strs.iter().map(|&x| String::from(x)).collect()
    }

    #[test]
    fn test_fl() {
        assert_eq!(
            Solution::longest_common_prefix(to_str_vec(vec!["flower", "flow", "flight"])),
            "fl"
        );
    }

    #[test]
    fn test_ch() {
        assert_eq!(
            Solution::longest_common_prefix(to_str_vec(vec!["church", "chuck", "chalk"])),
            "ch"
        );
    }

    #[test]
    fn test_no_common() {
        assert_eq!(
            Solution::longest_common_prefix(to_str_vec(vec!["dog", "racecar", "car"])),
            ""
        );
    }
}
