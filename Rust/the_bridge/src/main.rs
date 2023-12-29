use std::{io, str::FromStr, fmt::Debug};

const SAFE: char = '.';
const HOLE: char = '0';

const SPEED: &str = "SPEED";
const SLOW: &str = "SLOW";
const JUMP: &str = "JUMP";
const UP: &str = "UP";
const DOWN: &str = "DOWN";

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

trait Parse {
    fn parse<T: FromStr<Err = impl Debug>>() -> T {
        let mut input = String::new();
        let result = match io::stdin().read_line(&mut input) {
            Ok(v) => {
                let value = match input.trim().parse::<T>() {
                    Ok(value) => return value,
                    Err(error) => panic!("Input parse error: {:?}", error),
                };
            },
            Err(error) => panic!("Input read error: {error}"),
        };
    }
}

impl Parse for usize {}
impl Parse for String {}
impl Parse for u16 {}

fn parse_bikes(num_bikes: usize) -> Vec<(u16, u16, bool)> {
    let mut bikes: Vec<(u16, u16, bool)> = Vec::new();
    for i in 0..num_bikes as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], u16);
        let y = parse_input!(inputs[1], u16);
        let activated = parse_input!(inputs[2], u16) != 0;
        bikes.push((x, y, activated));
    }
    bikes
}

fn command(cmd: &str) {
    println!("{}", cmd);
}

fn main() {
    let num_bikes: usize = usize::parse();
    let req_bikes: usize = usize::parse();
    let l0: String = String::parse();
    let l1: String = String::parse();
    let l2: String = String::parse();
    let l3: String = String::parse();
    let mut bikes: Vec<(u16, u16, bool)> = Vec::new();
    let mut speed: u16 = 0;
    let mut counter: u16 = 0;

    eprintln!("num_bikes: {}", num_bikes);
    eprintln!("req_bikes: {}", req_bikes);
    eprintln!("l0: {}", l0);
    eprintln!("l1: {}", l1);
    eprintln!("l2: {}", l2);
    eprintln!("l3: {}", l3);

    loop {
        speed = u16::parse();
        bikes = parse_bikes(num_bikes);
        eprintln!("speed: {}", speed);
        eprintln!("bikes: {:?}", bikes);
        if counter == 0 {
            command(UP);
        } else {
            command(SPEED);
        }
        counter += 1;
    }
}
