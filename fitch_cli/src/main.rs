use fitch_syntax::parse_command;

// Just a small temporary cli for testing the parser

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() {
    loop {
        let line = read_line();
        let command = parse_command(&line);
        println!("{:?}", command);
    }
}
