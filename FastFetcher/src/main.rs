use std::env;
fn main() {

    let command_line_input: Vec<String> = env::args().skip(1).collect();

    println!("{:?}", command_line_input);
}
