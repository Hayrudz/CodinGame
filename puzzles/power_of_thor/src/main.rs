use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() -> ! {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let mut initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let mut initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _ = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.
    
        let direction_x = initial_tx - light_x;
        let direction_y = initial_ty - light_y;
    
        let direction = if direction_y > 0 {
            if direction_x > 0 {
                "NW"
            } else if direction_x < 0 {
                "NE"
            } else {
                "N"
            }
        } else if direction_y < 0 {
            if direction_x > 0 {
                "SW"
            } else if direction_x < 0 {
                "SE"
            } else {
                "S"
            }
        } else {
            if direction_x > 0 {
                "W"
            } else if direction_x < 0 {
                "E"
            } else {
                unreachable!()
            }
        };
    
        initial_tx += if direction_x > 0 { -1 } else if direction_x < 0 { 1 } else { 0 };
        initial_ty += if direction_y > 0 { -1 } else if direction_y < 0 { 1 } else { 0 };
    
        println!("{}", direction);
    }
}