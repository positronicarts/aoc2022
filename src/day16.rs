use std::collections::HashMap;

use regex::Regex;

pub struct Day16;

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Room {
    name: String,
    flow_rate: i32,
    adjoining: Vec<String>,
}

impl Room {
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"Valve (.*) has flow rate=(\d+); tunnel.* lead.* to valve.{0,1} (.*)")
            .unwrap();

        println!("Line {}", input);
        let name = re.captures_iter(input).next().unwrap()[1].to_string();
        let flow_rate = re.captures_iter(input).next().unwrap()[2].parse().unwrap();
        let adjoining = re.captures_iter(input).next().unwrap()[3]
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Self {
            name,
            flow_rate,
            adjoining,
        }
    }
}
impl aoc22::DayInner<Day16, i64> for Day16 {
    fn day(&self) -> i32 {
        16
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let rooms: Vec<Room> = input.lines().map(Room::parse).collect();
        let mut room_dict: HashMap<String, Room> = HashMap::new();

        for room in rooms.iter() {
            room_dict.insert(room.name.clone(), room.clone());
        }

        println!("Rooms {:?}", rooms);

        (1651, 0)
    }
}
