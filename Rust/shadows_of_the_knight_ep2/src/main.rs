use std::io;
use std::fmt::Display;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const COLDER: &str = "COLDER";
const WARMER: &str = "WARMER";
const SAME: &str = "SAME";
const UNKNOWN: &str = "UNKNOWN";

#[derive(Copy, Clone, PartialEq, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[inline(always)]
fn distance(coord1: &Coord, coord2: &Coord) -> u32 {
    (coord1.x - coord2.x).pow(2) + (coord1.y - coord2.y).pow(2)
}

fn calculate_center(coords: &Vec<Coord>, mut target_pos: Coord) -> Coord {
    let mut x = 0;
    let mut y = 0;
    for coord in coords {
        x += coord.x;
        y += coord.y;
    }
    target_pos = Coord {
        x: x / coords.len() as u32,
        y: y / coords.len() as u32,
    };
    target_pos
}

fn goto_and_update(cur_pos: &mut Coord, target_pos: &Coord) {
    println!("{}", target_pos);
    *cur_pos = *target_pos;
}

#[inline(always)]
fn distance_comparison(bomb_dir: &str, position: &Coord, prev_pos: &Coord, cur_pos: &Coord) -> bool {
    if bomb_dir == COLDER {
        return distance(position, prev_pos) < distance(position, cur_pos)
    }
    if bomb_dir == WARMER {
        return distance(position, prev_pos) > distance(position, cur_pos)
    }
    if bomb_dir == SAME {
        return distance(position, prev_pos) == distance(position, cur_pos)
    }
    false
}

fn update_possible_coords(possible_coords: &mut Vec<Coord>, bomb_dir: &str, prev_pos: &Coord, cur_pos: &Coord, w: u32, h: u32) {
    // if possible_coords.len() != 0 {
        possible_coords.retain(|&x| distance_comparison(bomb_dir, &x, prev_pos, cur_pos) && &x != cur_pos);
    // } else {
    //     for x in 0..w {
    //         for y in 0..h {
    //             let pos = Coord {x: x, y: y};
    //             if distance_comparison(bomb_dir, &pos, prev_pos, cur_pos) {
    //                 possible_coords.push(pos);
    //             }
    //         }
    //     }
    // }
}

fn determine_target_coord(possible_coords: &mut Vec<Coord>, cur_pos: &Coord) -> Coord {
    let avg_coord = calculate_center(possible_coords, *cur_pos);
    if possible_coords.contains(&avg_coord) {
        avg_coord
    } else {
        let mut distance_pos: Vec<(u32, Coord)> = Vec::new();
        for pos in possible_coords {
            distance_pos.push((distance(pos, &avg_coord), *pos));
        }
        distance_pos.sort_by(|x, y| x.0.cmp(&y.0));
        distance_pos[0].1
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], u32);
    let h = parse_input!(inputs[1], u32);

    let mut possible_coords: Vec<Coord> = Vec::new();
    for x in 0..w {
        for y in 0..h {
            possible_coords.push(Coord {x: x, y: y});
        }
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, u16);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut cur_pos = Coord {
        x: parse_input!(inputs[0], u32),
        y: parse_input!(inputs[1], u32),
    };

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    _ = input_line.trim();

    let mut target_pos = Coord {x: w/2, y: h/2};
    let mut prev_pos = cur_pos;

    goto_and_update(&mut cur_pos, &target_pos);

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim();
        eprintln!("bomb_dir: {}", bomb_dir);
        update_possible_coords(&mut possible_coords, &bomb_dir, &prev_pos, &cur_pos, w, h);
        target_pos = determine_target_coord(&mut possible_coords, &cur_pos);
        eprintln!("target pos: {:?}", target_pos);
        prev_pos = cur_pos;
        goto_and_update(&mut cur_pos, &target_pos);
    }
}
