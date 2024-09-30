use cli_utils::read_stdin;

fn main() {
    let buffer = read_stdin();

    println!("{}", buffer);
}
