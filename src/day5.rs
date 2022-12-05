use regex::Regex;

pub struct Day5;

fn day5_logic(mut lines: std::str::Lines, part: i32) -> String {
    // Initialise data structures
    let mut line = lines.next().unwrap();
    let num_cols = (line.len() + 1) / 4;
    let mut stacks = vec![];
    for _ in 0..num_cols {
        stacks.push(Vec::<char>::new());
    }

    // Populate starting state, a bit clunkily
    loop {
        if line.is_empty() {
            break;
        }
        for (ii, s) in stacks.iter_mut().enumerate().take(num_cols) {
            let c = line.chars().nth(ii * 4 + 1).unwrap();
            if c != ' ' {
                s.push(c);
            }
        }
        line = lines.next().unwrap();
    }

    // Handle the moves (in one of two ways)
    loop {
        let line_opt = lines.next();

        if let Some(line) = line_opt {
            let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
            let cap = re.captures_iter(line).next().unwrap();
            let num: usize = cap[1].parse().unwrap();
            let from: usize = cap[2].parse::<usize>().unwrap() - 1;
            let to: usize = cap[3].parse::<usize>().unwrap() - 1;
            for ii in 0..num {
                let from_index = match part {
                    1 => 0,
                    2 => num - ii - 1,
                    _ => panic!("Invalid part!"),
                };
                let c = stacks[from].remove(from_index);
                stacks[to].insert(0, c);
            }
        } else {
            break;
        }
    }

    stacks.iter().map(|s| s.first().unwrap()).collect()
}

impl aoc22::DayInner<Day5, String> for Day5 {
    fn day(&self) -> i32 {
        5
    }

    fn inner(&self, input: String) -> (String, String) {
        (day5_logic(input.lines(), 1), day5_logic(input.lines(), 2))
    }
}
