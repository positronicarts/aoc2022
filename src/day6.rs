pub struct Day6;

fn get_index(line: &str, length: usize) -> usize {
    let mut index = length - 1;
    let chars: Vec<_> = line.chars().collect();

    loop {
        let mut set = std::collections::HashSet::<char>::new();
        for c in chars.iter().take(index + 1).skip(index + 1 - length) {
            set.insert(*c);
        }

        if set.len() == length {
            return index + 1;
        }
        index += 1;
    }
}

impl aoc22::DayInner<Day6, Vec<usize>> for Day6 {
    fn day(&self) -> i32 {
        6
    }

    fn inner(&self, input: String) -> (Vec<usize>, Vec<usize>) {
        let lines: Vec<&str> = input.lines().collect();
        let p1 = lines.iter().map(|l| get_index(l, 4)).collect();
        let p2 = lines.iter().map(|l| get_index(l, 14)).collect();
        (p1, p2)
    }
}
