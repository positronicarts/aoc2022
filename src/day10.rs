use regex::Regex;

pub struct Day10;

#[derive(Debug)]
enum Command {
    Addx(i32),
    Noop,
    Unknown,
}

impl Command {
    fn parse(input: &str) -> Self {
        let addx_re = Regex::new(r"^addx (\S*)$").unwrap();
        let noop_re = Regex::new(r"^noop$").unwrap();

        if addx_re.is_match(input) {
            let v = addx_re.captures_iter(input).next().unwrap()[1]
                .parse()
                .unwrap();
            Command::Addx(v)
        } else if noop_re.is_match(input) {
            Command::Noop
        } else {
            Command::Unknown
        }
    }
}

impl aoc22::DayInner<Day10, i32> for Day10 {
    fn day(&self) -> i32 {
        10
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let commands: Vec<Command> = input.lines().map(Command::parse).collect();
        let mut values: Vec<i32> = vec![1];

        let mut signal_strength = 0;
        let mut x = 1;

        for command in commands {
            match command {
                Command::Addx(v) => {
                    values.push(x);
                    x += v;
                    values.push(x);
                }
                Command::Noop => {
                    values.push(x);
                }
                _ => panic!("Unknown command found!"),
            }
        }

        let mut index: i32 = 20;
        loop {
            if index as usize > values.len() {
                break;
            }
            signal_strength += values[index as usize - 1] * (index as i32);
            index += 40;
        }

        for (ii, val) in values.iter().enumerate() {
            if (ii as i32 % 40) >= (val - 1) && (ii as i32 % 40) <= (val + 1) {
                print!("#");
            } else {
                print!(".");
            }

            if ii % 40 == 39 {
                println!();
            }
        }

        (signal_strength, 0)
    }
}
