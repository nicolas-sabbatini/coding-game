use std::collections::HashMap;
use std::fmt;
use std::io;

// Could I use bitwise operators to do this?
// YES! I could! But this is more fun.
enum SignalBit {
    Low,
    High,
}
impl SignalBit {
    fn from_char(c: char) -> SignalBit {
        match c {
            '_' => SignalBit::Low,
            '-' => SignalBit::High,
            _ => panic!("Invalid signal"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            SignalBit::Low => '_',
            SignalBit::High => '-',
        }
    }

    fn and(&self, t: &SignalBit) -> SignalBit {
        match (self, t) {
            (SignalBit::Low, SignalBit::Low) => SignalBit::Low,
            (SignalBit::Low, SignalBit::High) => SignalBit::Low,
            (SignalBit::High, SignalBit::Low) => SignalBit::Low,
            (SignalBit::High, SignalBit::High) => SignalBit::High,
        }
    }

    fn or(&self, t: &SignalBit) -> SignalBit {
        match (self, t) {
            (SignalBit::Low, SignalBit::Low) => SignalBit::Low,
            (SignalBit::Low, SignalBit::High) => SignalBit::High,
            (SignalBit::High, SignalBit::Low) => SignalBit::High,
            (SignalBit::High, SignalBit::High) => SignalBit::High,
        }
    }

    fn xor(&self, t: &SignalBit) -> SignalBit {
        match (self, t) {
            (SignalBit::Low, SignalBit::Low) => SignalBit::Low,
            (SignalBit::Low, SignalBit::High) => SignalBit::High,
            (SignalBit::High, SignalBit::Low) => SignalBit::High,
            (SignalBit::High, SignalBit::High) => SignalBit::Low,
        }
    }

    fn not(&self) -> SignalBit {
        match self {
            SignalBit::Low => SignalBit::High,
            SignalBit::High => SignalBit::Low,
        }
    }
}

struct Signal {
    s: Vec<SignalBit>,
}
impl Signal {
    fn new(s: &str) -> Signal {
        Signal {
            s: s.chars().map(SignalBit::from_char).collect(),
        }
    }

    fn and(&self, t: &Signal) -> Signal {
        Signal {
            s: self
                .s
                .iter()
                .zip(t.s.iter())
                .map(|(a, b)| a.and(b))
                .collect(),
        }
    }

    fn or(&self, t: &Signal) -> Signal {
        Signal {
            s: self
                .s
                .iter()
                .zip(t.s.iter())
                .map(|(a, b)| a.or(b))
                .collect(),
        }
    }

    fn xor(&self, t: &Signal) -> Signal {
        Signal {
            s: self
                .s
                .iter()
                .zip(t.s.iter())
                .map(|(a, b)| a.xor(b))
                .collect(),
        }
    }

    fn not(&self) -> Signal {
        Signal {
            s: self.s.iter().map(SignalBit::not).collect(),
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.s.iter().map(SignalBit::to_char).collect::<String>()
        )
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = input_line.trim().parse::<usize>().unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = input_line.trim().parse::<usize>().unwrap();

    let inputs_signals: HashMap<String, Signal> = (0..n)
        .map(|_| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let input_name = inputs[0].trim().to_string();
            let input_signal = Signal::new(inputs[1].trim());
            (input_name, input_signal)
        })
        .collect();

    for _i in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let output_name = inputs[0].trim().to_string();
        let operation = inputs[1].trim().to_string();
        let input_name_1 = inputs[2].trim().to_string();
        let input_name_2 = inputs[3].trim().to_string();

        let i1 = inputs_signals.get(&input_name_1).unwrap();
        let i2 = inputs_signals.get(&input_name_2).unwrap();

        let output_signal = match operation.as_str() {
            "AND" => i1.and(i2),
            "OR" => i1.or(i2),
            "XOR" => i1.xor(i2),
            "NAND" => i1.and(i2).not(),
            "NOR" => i1.or(i2).not(),
            "NXOR" => i1.xor(i2).not(),
            _ => panic!("Invalid operation"),
        };

        println!("{} {}", output_name, output_signal);
    }
}
