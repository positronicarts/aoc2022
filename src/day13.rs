pub struct Day13;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Entry {
    List(Vec<Entry>),
    Number(i32),
}

impl Entry {
    fn parse(input: &str) -> Self {
        let rv = Self::parse_inner(&mut input.chars(), false);
        rv
    }

    fn parse_inner(input_iter: &mut dyn Iterator<Item = char>, mut in_list: bool) -> Self {
        let mut list = vec![];
        let mut next = input_iter.next().unwrap();

        loop {
            match next {
                '[' => {
                    if in_list {
                        list.push(Self::parse_inner(input_iter, true));
                    } else {
                        in_list = true;
                    }

                    next = input_iter.next().unwrap();
                }
                ']' => {
                    return Entry::List(list);
                }
                ',' => {
                    in_list = true;
                    next = input_iter.next().unwrap();
                }
                d => {
                    // Could be multi-digit...
                    let mut digit_list = vec![d];

                    next = input_iter.next().unwrap();

                    loop {
                        match next {
                            ']' | ',' => {
                                break;
                            }
                            d => {
                                digit_list.push(d);
                            }
                        }
                        next = input_iter.next().unwrap();
                    }

                    let digit_string: String = digit_list.iter().collect();
                    let digit = digit_string.parse::<i32>().unwrap();
                    list.push(Self::Number(digit));

                    if next == ']' {
                        if in_list || list.len() > 1 {
                            return Entry::List(list);
                        } else {
                            return Entry::Number(digit);
                        }
                    } else {
                        in_list = true;
                        next = input_iter.next().unwrap();
                    }
                }
            }
        }
    }

    fn cmp_inner(&self, other: &Self) -> i32 {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l - r,
            (Self::List(l), Self::List(r)) => {
                let mut r_iter = r.iter();
                let mut r_next = r_iter.next();

                for ll in l {
                    match r_next {
                        Some(rr) => {
                            let comp = ll.cmp_inner(rr);
                            if comp != 0 {
                                return comp;
                            }
                        }
                        None => {
                            return 1;
                        }
                    }
                    r_next = r_iter.next();
                }

                match r_next {
                    Some(_) => -1,
                    None => 0,
                }
            }
            (Self::Number(l), Self::List(_)) => Self::List(vec![Self::Number(*l)]).cmp_inner(other),
            (Self::List(_), Self::Number(r)) => self.cmp_inner(&Self::List(vec![Self::Number(*r)])),
        }
    }
}

impl PartialOrd for Entry {
    fn ge(&self, other: &Self) -> bool {
        self.cmp_inner(other) >= 0
    }
    fn gt(&self, other: &Self) -> bool {
        self.cmp_inner(other) > 0
    }
    fn le(&self, other: &Self) -> bool {
        self.cmp_inner(other) <= 0
    }
    fn lt(&self, other: &Self) -> bool {
        self.cmp_inner(other) < 0
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.cmp_inner(other) {
            d if d < 0 => Some(std::cmp::Ordering::Less),
            d if d > 0 => Some(std::cmp::Ordering::Greater),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Ord for Entry {
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl aoc22::DayInner<Day13, i32> for Day13 {
    fn day(&self) -> i32 {
        13
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut lines = input.lines();
        let mut index = 1;
        let mut count = 0;

        let dp1 = Entry::parse("[[2]]");
        let dp2 = Entry::parse("[[6]]");

        let mut entry_list = vec![dp1.clone(), dp2.clone()];

        loop {
            let lhs = Entry::parse(lines.next().unwrap());
            let rhs = Entry::parse(lines.next().unwrap());

            if lhs < rhs {
                count += index;
            }

            entry_list.push(lhs);
            entry_list.push(rhs);

            if lines.next().is_none() {
                break;
            }

            index += 1;
        }

        entry_list.sort();

        let mut prod: i32 = 1;

        for (ii, val) in entry_list.iter().enumerate() {
            if val == &dp1 || val == &dp2 {
                prod *= (ii + 1) as i32;
            }
        }

        (count, prod)
    }
}
