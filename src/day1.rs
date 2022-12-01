pub struct Day1;

impl aoc22::DayInner<Day1, i32> for Day1 {
    fn day(&self) -> i32 {
        1
    }

    fn inner(&self, input: String) -> (i32, i32) {

        // Read data - make sure we have a blank line at the end to check the final entries.
        let mut lines: Vec<&str> = input.lines().collect();
        lines.push("");

        // Initiate local variables
        let mut counter: i32 = 0;
        let mut max: i32 = 0;
        let mut maxes: [i32;3] = [0, 0, 0];

        // Brute force loop, updating where necessary
        for line in lines {
            if line.is_empty() {
                // End of block - see if we have a new max
                max = i32::max(max, counter);
                for (i, m) in maxes.iter().enumerate() {
                    if &counter > m {
                        for ii in (i+1..3).rev() {
                            maxes[ii] = maxes[ii-1];
                        }
                        maxes[i] = counter;
                        break;
                    }
                }
                counter = 0;
            } else {
                // Mid-block, update counter
                counter += line.parse::<i32>().unwrap();
            }
        }

        // And we're done!
        (max, maxes.iter().sum())
    }
}