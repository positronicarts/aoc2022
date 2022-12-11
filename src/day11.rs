use std::str::Lines;

use regex::Regex;

#[derive(Debug, Clone)]
enum MonkeyOp {
    Add(i64),
    Multiply(i64),
    Square,
    Double,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: MonkeyOp,
    test: i64,
    if_true: i64,
    if_false: i64,
    inspection_count: i64,
}

impl Monkey {
    fn parse(input: &mut Lines) -> Self {
        let header_re = Regex::new(r"^Monkey (\d*):$").unwrap();
        if !header_re.is_match(input.next().unwrap()) {
            panic!("No header row!")
        }

        let starting_re = Regex::new(r"^  Starting items: (.*)$").unwrap();
        let next = input.next().unwrap();
        let items: Vec<i64> = if !starting_re.is_match(next) {
            panic!("No starting row!")
        } else {
            starting_re.captures_iter(next).next().unwrap()[1]
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        };
        let operation_re = Regex::new(r"^  Operation: new = old (\S) (\S+)$").unwrap();
        let next = input.next().unwrap();
        let operation: MonkeyOp = if !operation_re.is_match(next) {
            panic!("No operation row! {}", next);
        } else {
            let op = &operation_re.captures_iter(next).next().unwrap()[1];
            let rhs = &operation_re.captures_iter(next).next().unwrap()[2];

            match (op, rhs) {
                ("*", "old") => MonkeyOp::Square,
                ("+", "old") => MonkeyOp::Double,
                ("*", d) => MonkeyOp::Multiply(d.parse().unwrap()),
                ("+", d) => MonkeyOp::Add(d.parse().unwrap()),
                _ => panic!("Don't know how to handle old {} {}", op, rhs),
            }
        };

        let test_re = Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
        let next = input.next().unwrap();
        let test = if !test_re.is_match(next) {
            panic!("No test row! {}", next);
        } else {
            test_re.captures_iter(next).next().unwrap()[1]
                .parse()
                .unwrap()
        };

        let true_re = Regex::new(r"^    If true: throw to monkey (\d+)$").unwrap();
        let next = input.next().unwrap();
        let if_true = if !true_re.is_match(next) {
            panic!("No if_true row! {}", next);
        } else {
            true_re.captures_iter(next).next().unwrap()[1]
                .parse()
                .unwrap()
        };

        let false_re = Regex::new(r"^    If false: throw to monkey (\d+)$").unwrap();
        let next = input.next().unwrap();
        let if_false = if !false_re.is_match(next) {
            panic!("No if_false row! {}", next);
        } else {
            false_re.captures_iter(next).next().unwrap()[1]
                .parse()
                .unwrap()
        };

        Monkey {
            items,
            operation,
            test,
            if_false,
            if_true,
            inspection_count: 0,
        }
    }

    fn process(&mut self, lcm: i64, divide_by_three: bool) -> Vec<(i64, i64)> {
        let mut rv = vec![];

        for ii in 0..self.items.len() {
            match self.operation {
                MonkeyOp::Square => {
                    self.items[ii] *= self.items[ii];
                }
                MonkeyOp::Double => {
                    self.items[ii] += self.items[ii];
                }
                MonkeyOp::Add(d) => {
                    self.items[ii] += d;
                }
                MonkeyOp::Multiply(d) => {
                    self.items[ii] *= d;
                }
            }

            if divide_by_three {
                self.items[ii] /= 3;
            }

            self.items[ii] %= lcm;

            let bool = (self.items[ii] % self.test) == 0;
            if bool {
                rv.push((self.items[ii], self.if_true));
            } else {
                rv.push((self.items[ii], self.if_false));
            }

            self.inspection_count += 1;
        }

        self.items = vec![];

        rv
    }
}

pub struct Day11;

impl aoc22::DayInner<Day11, i64> for Day11 {
    fn day(&self) -> i32 {
        11
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let mut lines = input.lines();
        let mut monkeys: Vec<Monkey> = vec![];

        loop {
            monkeys.push(Monkey::parse(&mut lines));

            if lines.next().is_none() {
                break;
            }
        }

        let lcm: i64 = monkeys.iter().map(|m| m.test).product();
        let mut p2_monkeys = monkeys.clone();

        for _ in 0..20 {
            for ii in 0..monkeys.len() {
                let rv = {
                    let monkey: &mut Monkey = &mut monkeys[ii];
                    monkey.process(lcm, true)
                };

                for (val, target) in rv {
                    let target: &mut Monkey = &mut monkeys[target as usize];
                    target.items.push(val);
                }
            }
        }

        let mut counts: Vec<i64> = monkeys.iter().map(|m| m.inspection_count).collect();
        counts.sort_unstable();
        counts.reverse();
        let p1 = counts[0] * counts[1];

        for _ in 0..10000 {
            for ii in 0..p2_monkeys.len() {
                let rv = {
                    let monkey: &mut Monkey = &mut p2_monkeys[ii];
                    monkey.process(lcm, false)
                };

                for (val, target) in rv {
                    let target: &mut Monkey = &mut p2_monkeys[target as usize];
                    target.items.push(val);
                }
            }
        }

        let mut counts: Vec<i64> = p2_monkeys.iter().map(|m| m.inspection_count).collect();
        counts.sort_unstable();
        counts.reverse();
        let p2 = counts[0] * counts[1];

        (p1, p2)
    }
}
