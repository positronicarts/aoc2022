mod day1;
mod day2;

use aoc22::Day;

fn main() {
    // day1::Day1 {}.run();
    day2::Day2 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, day2, Day};

    #[test]
    fn day1() {
        assert_eq!(day1::Day1 {}.test(), (24000, 45000));
    }

    #[test]
    fn day2() {
        assert_eq!(day2::Day2 {}.test(), (15, 0));
    }
}
