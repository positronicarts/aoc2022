use regex::Regex;

pub struct Day7;

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
    Unknown,
}

impl Command {
    fn parse(input: &str) -> Self {
        let cd_re = Regex::new(r"^\$ cd (\S*)$").unwrap();
        let ls_re = Regex::new(r"^\$ ls$").unwrap();

        if cd_re.is_match(input) {
            let dir_name = cd_re.captures_iter(input).next().unwrap()[1].to_string();
            Command::Cd(dir_name)
        } else if ls_re.is_match(input) {
            Command::Ls
        } else {
            Command::Unknown
        }
    }
}

#[derive(Debug)]
enum LsOutput {
    Dir(String),
    File(i64),
    Unknown,
}

impl LsOutput {
    fn parse(input: &str) -> Self {
        let dir_re = Regex::new(r"^dir (\S*)$").unwrap();
        let file_re = Regex::new(r"^(\d*) \S*$").unwrap();

        if dir_re.is_match(input) {
            let dir_name = dir_re.captures_iter(input).next().unwrap()[1].to_string();
            LsOutput::Dir(dir_name)
        } else if file_re.is_match(input) {
            let file_size = file_re.captures_iter(input).next().unwrap()[1]
                .parse()
                .unwrap();
            LsOutput::File(file_size)
        } else {
            LsOutput::Unknown
        }
    }
}

impl aoc22::DayInner<Day7, i64> for Day7 {
    fn day(&self) -> i32 {
        7
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let mut lines = input.lines();

        let mut size_list = Vec::<i64>::new();
        let mut path = Vec::<String>::new();
        let mut dir_sizes = Vec::<i64>::new();

        let mut line = lines.next().unwrap();

        loop {
            match Command::parse(line) {
                Command::Cd(dir) => {
                    if &dir == ".." {
                        path.pop().unwrap();
                        let dir_size = dir_sizes.pop().unwrap();

                        size_list.push(dir_size);

                        let index = dir_sizes.len() - 1;
                        dir_sizes[index] += dir_size;
                    } else {
                        path.push(dir);
                        dir_sizes.push(0);
                    }
                }
                Command::Ls => {
                    line = lines.next().unwrap_or_default();
                    loop {
                        match LsOutput::parse(line) {
                            LsOutput::Dir(_dir) => {
                                // Found a directory
                            }
                            LsOutput::File(file_size) => {
                                let index = dir_sizes.len() - 1;
                                dir_sizes[index] += file_size;
                            }
                            LsOutput::Unknown => {
                                break;
                            }
                        }
                        line = lines.next().unwrap_or_default();
                    }
                    continue;
                }
                Command::Unknown => {
                    break;
                }
            }

            line = lines.next().unwrap_or_default();
        }

        // Finish walking the tree...
        loop {
            path.pop().unwrap();
            let dir_size = dir_sizes.pop().unwrap();

            size_list.push(dir_size);

            if path.is_empty() {
                // Got to the root
                break;
            }

            let index = dir_sizes.len() - 1;
            dir_sizes[index] += dir_size;
        }

        // Sort output
        let total_size = size_list[size_list.len() - 1];
        let free_size = 70000000 - total_size;
        let needed_size = 30000000 - free_size;

        // Get actual answers out
        let p1 = size_list.iter().filter(|s| *s < &100000).sum();
        let p2 = size_list
            .iter()
            .filter(|s| *s > &needed_size)
            .min()
            .unwrap();

        (p1, *p2)
    }
}
