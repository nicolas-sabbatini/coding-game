use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

enum Follow {
    Left,
    Right,
}

enum Looking {
    Left,
    Right,
    Bottom,
    Top,
    Unknown,
}
impl Looking {
    fn from_char(c: char) -> Looking {
        match c {
            '<' => Looking::Left,
            '>' => Looking::Right,
            '^' => Looking::Top,
            'v' => Looking::Bottom,
            _ => panic!("unknown input"),
        }
    }

    fn rotate_clockwise(&self) -> Looking {
        match self {
            Looking::Left => Looking::Top,
            Looking::Top => Looking::Right,
            Looking::Right => Looking::Bottom,
            Looking::Bottom => Looking::Left,
            Looking::Unknown => panic!("unknown looking"),
        }
    }

    fn rotate_counter_clockwise(&self) -> Looking {
        match self {
            Looking::Left => Looking::Bottom,
            Looking::Bottom => Looking::Right,
            Looking::Right => Looking::Top,
            Looking::Top => Looking::Left,
            Looking::Unknown => panic!("unknown looking"),
        }
    }

    fn follow(&self, follow: &Follow) -> Looking {
        match follow {
            Follow::Left => self.rotate_counter_clockwise(),
            Follow::Right => self.rotate_clockwise(),
        }
    }

    fn unfollow(&self, follow: &Follow) -> Looking {
        match follow {
            Follow::Left => self.rotate_clockwise(),
            Follow::Right => self.rotate_counter_clockwise(),
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Looking::Left => (-1, 0),
            Looking::Right => (1, 0),
            Looking::Top => (0, -1),
            Looking::Bottom => (0, 1),
            Looking::Unknown => panic!("unknown looking"),
        }
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize);
    let height = parse_input!(inputs[1], usize);

    let mut room = vec![vec![0; height]; width];
    let mut looking = Looking::Unknown;
    let mut start = (0, 0);

    for y in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim().to_string();
        for (x, c) in line.chars().enumerate() {
            match c {
                '0' => room[x][y] = 0,
                '#' => room[x][y] = -1,
                _ => {
                    room[x][y] = 0;
                    looking = Looking::from_char(c);
                    start = (x, y);
                }
            }
        }
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let side = match input_line.trim().chars().next().unwrap() {
        'L' => Follow::Left,
        'R' => Follow::Right,
        _ => panic!("unknown input"),
    };

    let mut current = start;

    loop {
        let next = calculate_next(&room, width, height, &mut looking, &side, &current);
        match next {
            None => break,
            Some(next) => {
                room[current.0][current.1] += 1;
                if next == start {
                    break;
                }
                current = next;
            }
        }
    }

    for y in 0..height {
        for row in &room {
            if row[y] > -1 {
                print!("{}", row[y]);
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn can_move(t: (i32, i32), map: &[Vec<i32>], width: usize, height: usize) -> bool {
    t.0 >= 0
        && t.0 < width as i32
        && t.1 >= 0
        && t.1 < height as i32
        && map[t.0 as usize][t.1 as usize] >= 0
}

fn calculate_next(
    map: &[Vec<i32>],
    width: usize,
    height: usize,
    looking: &mut Looking,
    side: &Follow,
    start: &(usize, usize),
) -> Option<(usize, usize)> {
    let check = looking.follow(side).offset();
    let next = (start.0 as i32 + check.0, start.1 as i32 + check.1);
    if can_move(next, map, width, height) {
        *looking = looking.follow(side);
        return Some((next.0 as usize, next.1 as usize));
    }
    for _i in 0..4 {
        let check = looking.offset();
        let next = (start.0 as i32 + check.0, start.1 as i32 + check.1);
        if can_move(next, map, width, height) {
            return Some((next.0 as usize, next.1 as usize));
        }
        *looking = looking.unfollow(side);
    }
    None
}
