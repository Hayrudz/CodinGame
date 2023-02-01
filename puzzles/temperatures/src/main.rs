use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let mut temperatures = Vec::new();
    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);
        temperatures.push(t);
    }

    let mut closest = 0;
    let mut closest_diff = i32::MAX;
    for &temperature in temperatures.iter() {
        let diff = (0 - temperature).abs();
        if diff < closest_diff {
            closest_diff = diff;
            closest = temperature;
        } else if diff == closest_diff {
            closest = temperature.max(closest);
        }
    }

    if closest_diff == i32::MAX {
        println!("0");
    } else {
        println!("{}", closest);
    }
}