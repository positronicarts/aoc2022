use regex::Regex;

pub struct Day15;

#[derive(Debug)]
struct Sensor {
    location: (i32, i32),
    nearest_becon: (i32, i32),
}

#[derive(PartialEq)]
enum LocationType {
    S,
    B,
    Maybe,
    No,
}

impl Sensor {
    fn parse(input: &str) -> Self {
        let re =
            Regex::new(r"^Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)$").unwrap();
        let captures = re.captures_iter(input).next().unwrap();
        Sensor {
            location: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            nearest_becon: (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        }
    }
}
impl aoc22::DayInner<Day15, i64> for Day15 {
    fn day(&self) -> i32 {
        15
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let sensors: Vec<Sensor> = input.lines().map(Sensor::parse).collect();
        let test = sensors[0].location.0 == 2;
        let maxv = if test { 20 } else { 4000000 };
        let test_row = if test { 10 } else { 2000000 };
        let mut xmin = sensors
            .iter()
            .map(|s| std::cmp::min(s.location.0, s.nearest_becon.0))
            .min()
            .unwrap();
        let mut xmax = sensors
            .iter()
            .map(|s| std::cmp::max(s.location.0, s.nearest_becon.0))
            .max()
            .unwrap();
        let dist = xmax - xmin;
        xmin -= dist;
        xmax += dist;

        let ymin = 0;
        let ymax = maxv;

        let mut p2: i64 = 0;

        let get_type = |check_x: i32, check_y: i32| {
            let mut maybe = true;
            for sensor in sensors.iter() {
                if check_x == sensor.location.0 && check_y == sensor.location.1 {
                    return LocationType::S;
                }
                if check_x == sensor.nearest_becon.0 && check_y == sensor.nearest_becon.1 {
                    return LocationType::B;
                }
                if (check_x - sensor.location.0).abs() + (check_y - sensor.location.1).abs()
                    <= (sensor.location.0 - sensor.nearest_becon.0).abs()
                        + (sensor.location.1 - sensor.nearest_becon.1).abs()
                {
                    maybe = false;
                }
            }

            if maybe {
                LocationType::Maybe
            } else {
                LocationType::No
            }
        };

        let count = (xmin..xmax)
            .map(|testx| get_type(testx, test_row))
            .filter(|c| c == &LocationType::No)
            .count();

        // Given uniqueness, check near the boundary of each sensor.
        xmin = std::cmp::max(0, xmin);
        xmax = std::cmp::min(maxv, xmax);

        'outer: for sensor in sensors.iter() {
            let dist = (sensor.location.0 - sensor.nearest_becon.0).abs()
                + (sensor.location.1 - sensor.nearest_becon.1).abs();

            for ii in 0..dist {
                let trial_x = sensor.location.0 - dist + ii;
                let trial_y = sensor.location.1 + ii;

                for dx in -1..2 {
                    {
                        let dy = 0;
                        let testx = trial_x + dx;
                        let testy = trial_y + dy;

                        if testx > xmin
                            && testx < xmax
                            && testy > ymin
                            && testy < ymax
                            && get_type(testx, testy) == LocationType::Maybe
                        {
                            p2 = 4000000 * testx as i64 + testy as i64;
                            break 'outer;
                        }
                    }
                }

                let trial_x = sensor.location.0 - dist + ii;
                let trial_y = sensor.location.1 - ii;

                for dx in -1..2 {
                    {
                        let dy = 0;
                        let testx = trial_x + dx;
                        let testy = trial_y + dy;

                        if testx > xmin
                            && testx < xmax
                            && testy > ymin
                            && testy < ymax
                            && get_type(testx, testy) == LocationType::Maybe
                        {
                            p2 = 4000000 * testx as i64 + testy as i64;
                            break 'outer;
                        }
                    }
                }

                let trial_x = sensor.location.0 + dist - ii;
                let trial_y = sensor.location.1 - ii;

                for dx in -1..2 {
                    {
                        let dy = 0;
                        let testx = trial_x + dx;
                        let testy = trial_y + dy;

                        if testx > xmin
                            && testx < xmax
                            && testy > ymin
                            && testy < ymax
                            && get_type(testx, testy) == LocationType::Maybe
                        {
                            p2 = 4000000 * testx as i64 + testy as i64;
                            break 'outer;
                        }
                    }
                }

                let trial_x = sensor.location.0 + dist - ii;
                let trial_y = sensor.location.1 - ii;

                for dx in -1..2 {
                    {
                        let dy = 0;
                        let testx = trial_x + dx;
                        let testy = trial_y + dy;

                        if testx > xmin
                            && testx < xmax
                            && testy > ymin
                            && testy < ymax
                            && get_type(testx, testy) == LocationType::Maybe
                        {
                            p2 = 4000000 * testx as i64 + testy as i64;
                            break 'outer;
                        }
                    }
                }
            }
        }

        (count as i64, p2)
    }
}
