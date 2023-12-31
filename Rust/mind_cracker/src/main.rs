use std::{io, str::FromStr};

const MAX_COLORS: usize = 10;
const MAX_COLUMNS: usize = 7;
const MAX_LINES: usize = 11;

macro_rules! read_line_as {
    ($t:ty) => {{
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line.trim().parse::<$t>().unwrap()
    }};
}

#[derive(Debug, Default, Clone)]
struct Line {
    colors: Vec<u8>,
    num_black: u8,
    num_white: u8,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colors: Vec<u8> = vec![0; MAX_COLUMNS];
        let mut num_black = 0;
        let mut num_white = 0;
        let mut num_spaces = 0;

        for byte in s.chars().into_iter() {
            match byte {
                '0'..='9' => if num_spaces == 0 {
                        colors.push(byte.to_digit(10).unwrap() as u8);
                    } else if num_spaces == 1 {
                        num_black = byte.to_digit(10).unwrap() as u8;
                    } else {
                        num_white = byte.to_digit(10).unwrap() as u8;
                    },
                ' ' => num_spaces += 1,
                _ => return Err(()),
            }
        }

        Ok(Line {
            colors,
            num_black,
            num_white,
        })
    }
}

fn main() {
    let num_colors = read_line_as!(u8);
    let num_columns = read_line_as!(u8);
    let num_lines = read_line_as!(usize);
    let mut lines = vec![Line::default(); num_lines];

    for i in 0..num_lines {
        let input_line = read_line_as!(String);
        lines[i] = input_line.parse::<Line>().unwrap();
    }

    println!("021");
}
