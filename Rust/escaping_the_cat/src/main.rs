use std::io;
use std::fmt::Display;
use std::f32::consts::PI;

const MIN_MOUSE_CAT_DISTANCE: f32 = 80.0;
const POOL_RADIUS: f32 = 500.0;
const MOUSE_SPEED: f32 = 10.0;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

fn command(target_coord: &Coord, message: &str) {
    println!("{} {}", target_coord, message);
}

#[inline(always)]
fn distance(coord1: &Coord, coord2: &Coord) -> f32 {
    (((coord1.x - coord2.x).pow(2) + (coord1.y - coord2.y).pow(2)) as f32).sqrt()
}

fn angle_between_points(coord1: &Coord, coord2: &Coord) -> f32 {
    let theta1 = (coord1.y as f32).atan2(coord1.x as f32);
    let theta2 = (coord2.y as f32).atan2(coord2.x as f32);

    (theta1-theta2).abs() % PI
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let cat_speed = parse_input!(input_line, f32);
    let speed_ratio = cat_speed / MOUSE_SPEED;
    let max_circling_radius = POOL_RADIUS / speed_ratio;
    let min_dashing_radius = (POOL_RADIUS * (1.0 - PI / speed_ratio)).max(0.0);
    let sweet_spot_radius = max_circling_radius - 1.0;
    let mut start_position_reached = false;
    let start_position = Coord {x: sweet_spot_radius as i32, y: 0};
    let mut sweet_spot_reached = false;
    let mut counter = 0;
    let circling_coords = vec![
        Coord {x: 0, y: -sweet_spot_radius as i32},
        Coord {x: -sweet_spot_radius as i32, y: 0},
        Coord {x: 0, y: sweet_spot_radius as i32},
        Coord {x: sweet_spot_radius as i32, y: 0},
    ];
    let mut dash_target = Coord {x: 0, y: 0};
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let mouse_coord = Coord {
            x: parse_input!(inputs[0], i32),
            y: parse_input!(inputs[1], i32),
        };

        let cat_coord = Coord {
            x: parse_input!(inputs[2], i32),
            y: parse_input!(inputs[3], i32),
        };

        eprintln!("speed ratio: {}", speed_ratio);
        eprintln!("max circling radius: {}", max_circling_radius);
        eprintln!("min dashing radius: {}", min_dashing_radius);
        eprintln!("sweet spot radius: {}", sweet_spot_radius);

        if mouse_coord == start_position {
            start_position_reached = true;
        }

        let theta = angle_between_points(&mouse_coord, &cat_coord);
        eprintln!("angle: {}", theta);
        if 3.1 < theta {
            sweet_spot_reached = true;
        }

        if !start_position_reached {
            command(&start_position, "go to start position");
        } else if !sweet_spot_reached {
            if circling_coords[counter%4] != mouse_coord {
                command(&circling_coords[counter%4], "circling");
            } else {
                counter += 1;
                command(&circling_coords[counter%4], "circling");
            }
        } else {
            if dash_target == (Coord {x: 0, y: 0}) {
                dash_target = Coord {x: -2*cat_coord.x, y: -2*cat_coord.y};
            }
            command(&dash_target, "dash for it!");
        }


    }
}
