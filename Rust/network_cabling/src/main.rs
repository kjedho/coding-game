use std::io;

#[derive(Debug, Clone)]
struct Building {
    x: i32,
    y: i32,
}

fn parse_input() -> Vec<Building> {
    let mut num_buildings = String::new();
    io::stdin().read_line(&mut num_buildings).expect("Failed to read line");
    eprintln!("{}", num_buildings);
    let mut buildings: Vec<Building> = Vec::new();
    let num_buildings = num_buildings.trim().parse::<i32>().unwrap();
    for _ in 0..num_buildings {
        let mut building = String::new();
        io::stdin().read_line(&mut building).expect("Failed to read line");
        eprintln!("{:?}", building.replace("\n", ""));
        let mut building = building.trim().split_whitespace();
        let x = building.next().unwrap().parse::<i32>().unwrap();
        let y = building.next().unwrap().parse::<i32>().unwrap();
        buildings.push(Building { x, y });
    }
    buildings
}

fn median(buildings: &Vec<Building>) -> i32 {
    let mut buildings = buildings.clone();
    buildings.sort_by(|b1, b2| (b1.y).cmp(&b2.y));
    eprintln!("{:?}", buildings);

    let mid = buildings.len() / 2;
    if buildings.len() % 2 == 0 {
        if mid != 0 {
            (buildings[mid-1].y + buildings[mid].y) / 2
        }
        else {
            0
        }
    } else {
        buildings[mid].y
    }
}

fn determine_center_cable(buildings: &Vec<Building>) -> (i32, i32, i32) {
    let mut x_min = buildings[0].x;
    let mut x_max = buildings[0].x;

    for building in buildings {
        if building.x < x_min {
            x_min = building.x;
        }
        if building.x > x_max {
            x_max = building.x;
        }
    }
    let y_median = median(buildings);
    eprintln!("{:?}", (x_min, x_max, y_median));
    (x_min, x_max, y_median)
}

fn determine_vertical_cable_length(buildings: Vec<Building>, y_median: i32) -> i32 {
    let mut vertical_cable_length = 0;
    for building in buildings {
        vertical_cable_length += (building.y - y_median).abs();
    }
    vertical_cable_length
}

fn main() {
    let buildings = parse_input();
    let (x_min, x_max, y_median) = determine_center_cable(&buildings);
    let center_cable_length = x_max - x_min;
    let total_length = center_cable_length + determine_vertical_cable_length(buildings, y_median);
    println!("{:?}", total_length);
}
