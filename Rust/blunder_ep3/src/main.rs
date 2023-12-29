use std::io;
use core::f64::MAX;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn o_1(n: i64) -> f64 {
    1.0
}

fn o_n(n: i64) -> f64 {
    n as f64
}

fn o_n2(n: i64) -> f64 {
    (n*n) as f64
}

fn o_n3(n: i64) -> f64 {
    (n*n*n) as f64
}

fn o_log_n(n: i64) -> f64 {
    (n as f64).log2()
}

fn o_n_log_n(n: i64) -> f64 {
    n as f64*(n as f64).log2()
}

fn o_n2_log_n(n: i64) -> f64 {
    (n*n) as f64*(n as f64).log2()
}

fn o_2_n(n: i64) -> f64 {
    (2.0 as f64).powf(n as f64)
}

fn mean(data: &Vec<f64>) -> f64 {
    let mut mean: f64 = 0.0;
    for d in data {
        mean += d/data.len() as f64;
    }
    mean
}

fn variance(data: &Vec<f64>, mean: f64) -> f64 {
    let mut variance: f64 = 0.0;
    for d in data {
        variance += ((d-mean)*(d-mean))/data.len() as f64;
    }
    variance
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    eprintln!("n: {}", n);
    let mut data_points = Vec::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let num = parse_input!(inputs[0], i64);
        let time = parse_input!(inputs[1], f64);
        data_points.push((num, time));
    }
    let mappings: Vec<(&str, fn(i64) -> f64)> = vec![
        ("O(1)", o_1),
        ("O(n)", o_n),
        ("O(n^2)", o_n2),
        ("O(n^3)", o_n3),
        ("O(log n)", o_log_n),
        ("O(n log n)", o_n_log_n),
        ("O(n^2 log n)", o_n2_log_n),
    ];
    let mut best_fit = "";
    let mut min_normalized_variance = MAX;
    let mut ratios = Vec::new();
    for (name, function) in mappings.iter() {
        for (num, time) in data_points.iter() {
            ratios.push(time/function(*num));
        }
        let mean = mean(&ratios);
        let variance = variance(&ratios, mean);
        let normalized_variance = variance/(mean*mean);
        if normalized_variance < min_normalized_variance {
            min_normalized_variance = normalized_variance;
            best_fit = name;
        }
    }
    println!("{}", best_fit);
}
