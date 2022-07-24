use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    let t1 = parse_input!(inputs[2], f32);
    let t2 = parse_input!(inputs[3], f32);
    let t3 = parse_input!(inputs[4], f32);

    let mut pic1: HashMap<char, (f32, f32)> = HashMap::new();
    let mut pic2: HashMap<char, (f32, f32)> = HashMap::new();
    let mut asteroid_name: Vec<char> = Vec::new();

    for y in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();

        inputs[0]
            .trim()
            .chars()
            .enumerate()
            .for_each(|(x, chr)| match chr {
                '.' => (),
                _ => {
                    pic1.insert(chr, (x as f32, y as f32));
                    asteroid_name.push(chr);
                }
            });
        inputs[1]
            .trim()
            .chars()
            .enumerate()
            .for_each(|(x, chr)| match chr {
                '.' => (),
                _ => {
                    pic2.insert(chr, (x as f32, y as f32));
                }
            });
    }

    asteroid_name.sort();

    let mut end_state: HashMap<(isize, isize), char> = HashMap::new();

    let time_step = t2 - t1;
    let time_end = t3 - t2;

    for chr in &asteroid_name {
        let (sx, sy) = pic1.get(chr).unwrap();
        let (ex, ey) = pic2.get(chr).unwrap();
        let step_x = (ex - sx) / time_step;
        let step_y = (ey - sy) / time_step;
        let end_position = (
            (step_x * time_end).floor() as isize + *ex as isize,
            (step_y * time_end).floor() as isize + *ey as isize,
        );
        if end_state.get(&end_position).is_none() {
            end_state.insert(end_position, *chr);
        }
    }

    for y in 0..h as isize {
        for x in 0..w as isize {
            match end_state.get(&(x, y)) {
                Some(chr) => {
                    print!("{}", chr);
                }
                None => {
                    print!(".");
                }
            }
        }
        println!();
    }
}
