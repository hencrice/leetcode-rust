pub struct Solution {}

struct NumAndCount {
    // the current number we are counting
    num: u32,
    // how many `num` are there
    count: u32,
}

impl NumAndCount {
    fn new(num: u32) -> Self {
        NumAndCount { num, count: 1 }
    }

    fn inc(&mut self) {
        self.count += 1;
    }
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            // directly return the result for the base case
            return String::from("1");
        }

        // for the other cases, first recursively obtain the previous count_and_say result
        let prev_count = Solution::count_and_say(n - 1);
        let nums_and_counts: Vec<NumAndCount> = prev_count
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold(Vec::with_capacity(prev_count.len()), |mut acc, x| {
                match acc.last_mut() {
                    Some(n) => {
                        if n.num == x {
                            // we are still accumulating the same number
                            n.inc();
                        } else {
                            // start accumulate for a new number
                            acc.push(NumAndCount::new(x));
                        }
                        acc
                    }
                    None => {
                        // this is the first number we've ever encountered
                        acc.push(NumAndCount::new(x));
                        acc
                    }
                }
            });
        let mut res = String::new();
        for nc in nums_and_counts {
            res.push_str(&nc.count.to_string());
            res.push_str(&nc.num.to_string());
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base_case() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_and_say(2), "11");
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::count_and_say(3), "21");
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::count_and_say(5), "111221");
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::count_and_say(8), "1113213211");
    }
}
