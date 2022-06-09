use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(PartialEq)]
enum Arrangement {
    Series,
    Parallel,
    Undefined,
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let resistors: HashMap<String, f32> = (0..n)
        .map(|_| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let name = inputs[0].trim().to_string();
            let r = parse_input!(inputs[1], f32);
            (name, r)
        })
        .collect();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let circuit = input_line.split(' ').collect::<Vec<_>>();

    println!("{:.1}", circuit_resistance(&resistors, &circuit));
}

fn circuit_resistance(r: &HashMap<String, f32>, c: &Vec<&str>) -> f32 {
    let (_, res) = circuit_resistance_rec(r, c, 0);
    res
}

fn circuit_resistance_rec(r: &HashMap<String, f32>, c: &Vec<&str>, mut i: usize) -> (usize, f32) {
    let mut resistors: Vec<f32> = Vec::new();
    let mut arrangement = Arrangement::Undefined;

    while i < c.len() {
        match c[i] {
            "(" if arrangement == Arrangement::Undefined => arrangement = Arrangement::Series,
            "[" if arrangement == Arrangement::Undefined => arrangement = Arrangement::Parallel,
            "(" | "[" => {
                let (new_i, res) = circuit_resistance_rec(r, c, i);
                i = new_i;
                resistors.push(res);
            }
            ")" => break,
            "]" => break,
            _ => {
                let r = r.get(c[i]).unwrap();
                resistors.push(*r);
            }
        }
        i += 1;
    }

    match arrangement {
        Arrangement::Series => (i, resistors.iter().sum()),
        Arrangement::Parallel => (i, 1.0 / resistors.iter().map(|r| 1.0 / r).sum::<f32>()),
        Arrangement::Undefined => panic!("Undefined"),
    }
}
