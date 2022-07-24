use std::io;

fn parse_command(str: String) {
    let mut chars = str.chars().collect::<Vec<char>>();
    let last = chars.pop().unwrap();
    let before_last = chars.pop().unwrap();
    let p = match (before_last, last) {
        ('n', 'l') => '\n',
        ('s', 'p') => ' ',
        ('b', 'S') => '\\',
        ('s', 'Q') => '\'',
        _ => {
            chars.push(before_last);
            last
        }
    };
    let n = chars
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(1);
    print!("{}", p.to_string().repeat(n));
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
        parse_command(command);
    }
}
