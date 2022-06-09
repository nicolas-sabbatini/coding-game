use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize);
    let height = parse_input!(inputs[1], usize);

    let mut map: Vec<Vec<i32>> = (0..height)
        .map(|_| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            input_line
                .trim()
                .chars()
                .map(|c| if c == '#' { -1 } else { 0 })
                .collect()
        })
        .collect();

    for x in 0..width {
        for y in 0..height {
            if map[y][x] == -1 {
                continue;
            }
            if x != 0 && map[y][x - 1] != -1 {
                map[y][x] += 1;
            }
            if x != width - 1 && map[y][x + 1] != -1 {
                map[y][x] += 1;
            }
            if y != 0 && map[y - 1][x] != -1 {
                map[y][x] += 1;
            }
            if y != height - 1 && map[y + 1][x] != -1 {
                map[y][x] += 1;
            }
        }
    }

    map.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|x| if *x == -1 {
                    "#".to_owned()
                } else {
                    x.to_string()
                })
                .collect::<String>()
        );
    });
}
