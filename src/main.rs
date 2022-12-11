mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc22::Day;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2 {}.run();
    // day3::Day3 {}.run();
    // day4::Day4 {}.run();
    // day5::Day5 {}.run();
    // day6::Day6 {}.run();
    // day7::Day7 {}.run();
    // day8::Day8 {}.run();
    // day9::Day9 {}.run();
    // day10::Day10 {}.run();
    day11::Day11 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, day10, day11, day2, day3, day4, day5, day6, day7, day8, day9, Day};

    #[test]
    fn day1() {
        assert_eq!(day1::Day1 {}.test(), (24000, 45000));
    }

    #[test]
    fn day2() {
        assert_eq!(day2::Day2 {}.test(), (15, 12));
    }

    #[test]
    fn day3() {
        assert_eq!(day3::Day3 {}.test(), (157, 70));
    }

    #[test]
    fn day4() {
        assert_eq!(day4::Day4 {}.test(), (2, 4));
    }

    #[test]
    fn day5() {
        assert_eq!(day5::Day5 {}.test(), ("CMZ".to_string(), "MCD".to_string()));
    }

    #[test]
    fn day6() {
        assert_eq!(
            day6::Day6 {}.test(),
            (vec![7, 5, 6, 10, 11], vec![19, 23, 23, 29, 26])
        );
    }

    #[test]
    fn day7() {
        assert_eq!(day7::Day7 {}.test(), (95437, 24933642));
    }

    #[test]
    fn day8() {
        assert_eq!(day8::Day8 {}.test(), (21, 8));
    }

    #[test]
    fn day9() {
        assert_eq!(day9::Day9 {}.test(), (13, 1));
    }

    #[test]
    fn day10() {
        assert_eq!(day10::Day10 {}.test(), (13140, 0));
    }

    #[test]
    fn day11() {
        assert_eq!(day11::Day11 {}.test(), (10605, 2713310158));
    }
}
