use std::collections::HashMap;

use regex::Regex;

pub struct Day16;

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Room {
    name: String,
    flow_rate: i32,
    adjoining: Vec<String>,
    distance_to_others: HashMap<String, i32>,
}

impl Room {
    fn parse(input: &str) -> Self {
        let re = Regex::new(
            r"Valve (.*) has flow rate=(\d+); tunnel.{0,1} lead.{0,1} to valve.{0,1} (.*)",
        )
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
            ..Default::default()
        }
    }

    fn build_distance_map(&mut self, room_dict: &HashMap<String, Room>) {
        let mut open_set = vec![&self.name];
        self.distance_to_others.insert(self.name.clone(), 0);

        while !open_set.is_empty() {
            let next = open_set.remove(0);
            for adj in room_dict.get(next).unwrap().adjoining.iter() {
                if !self.distance_to_others.contains_key(adj) {
                    self.distance_to_others
                        .insert(adj.clone(), self.distance_to_others[next] + 1);
                    open_set.push(adj);
                }
            }
        }
    }

    fn walk(
        room_dict: &HashMap<String, Room>,
        from: &Room,
        mut closed_list: Vec<String>,
        remaining_rooms: Vec<&String>,
        remaining_time: i32,
        depth: i32,
    ) -> i32 {
        // println!("Walking through {:?}, depth {}, time left {}", from, depth, remaining_time);
        // if depth > 1     {
        //     return 0;
        // }
        let mut best: i32 = 0;
        closed_list.push(from.name.clone());

        for next_room_name in remaining_rooms.iter() {
            // print!("Possible next room {}", next_room_name);
            let next_room = room_dict[&(*next_room_name).clone()].clone();
            // println!(" Got room");
            // println!(" Distances are {:?}", from.distance_to_others);
            let left_time = remaining_time - from.distance_to_others[&next_room.name] - 1;
            if left_time > 0 && !closed_list.contains(next_room_name) {
                // println!(" - trying");
                // let time_left_after = remaining_time - from.distance_to_others[&next_room.name] - 1;
                let new_remaining_rooms = remaining_rooms
                    .iter()
                    .filter(|r| *r != next_room_name)
                    .copied()
                    .collect();
                let candidate = Self::walk(
                    room_dict,
                    &next_room,
                    closed_list.clone(),
                    new_remaining_rooms,
                    left_time,
                    depth + 1,
                ) + (next_room.flow_rate * left_time);
                if candidate > best {
                    best = candidate;
                }
            } else {
                // println!(" - skipping");
            }
        }

        best
    }
}

impl aoc22::DayInner<Day16, i32> for Day16 {
    fn day(&self) -> i32 {
        16
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let rooms: Vec<Room> = input.lines().map(Room::parse).collect();
        println!("There are {} rooms", rooms.len());

        let clone = rooms.clone();
        let room_names: Vec<&String> = clone.iter().map(|r| &r.name).collect();
        let mut room_dict: HashMap<String, Room> = HashMap::new();

        for room in rooms.into_iter() {
            room_dict.insert(room.name.clone(), room);
        }

        // Fill out the distance map for each room
        // let mut rooms: Vec<&Room> = room_dict.values().collect();

        // for ii in 0..rooms.len() {
        //     let room = rooms.get_mut(ii).unwrap();
        //     room.build_distance_map(&room_dict);
        // }
        let clone_dict = room_dict.clone();
        for room in room_dict.values_mut() {
            room.build_distance_map(&clone_dict);
        }

        // println!("Rooms {:?}", rooms);
        println!("Room dictionary {:?}", room_dict);
        // return (0,0);

        let p1 = Room::walk(&room_dict, &room_dict["AA"], vec![], room_names, 30, 0);

        (p1, 0)
    }
}
