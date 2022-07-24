use std::io;

enum Command {
    EndLine,
    Char(usize, String),
}

impl From<String> for Command {
    fn from(str: String) -> Self {
        let s = &str[..str.len() - 1];
        let rep = s.chars().take_while(|c| c.is_numeric()).collect::<String>();
        if rep.is_empty() {
            return Command::EndLine;
        }
        let n = rep.parse::<usize>().unwrap();
        if rep.len() == (str.len() - 1) {
            return Command::Char(n, String::from(&str[str.len() - 1..]));
        }
        match &str[str.len() - 2..] {
            "sp" => Command::Char(n, " ".to_string()),
            "bS" => Command::Char(n, "\\".to_string()),
            "sQ" => Command::Char(n, "'".to_string()),
            _ => panic!("Err"),
        }
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let input = input_line
        .trim_matches('\n')
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    for command in input {
        match Command::from(command) {
            Command::EndLine => println!(),
            Command::Char(n, s) => print!("{}", s.repeat(n)),
        }
    }
}
