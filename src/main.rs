mod my_sync_arg_parser;

fn main() {
    let user_instructions = my_sync_arg_parser::parse_args(std::env::args().skip(1));

    println!("user_instructions=\n{}\n", user_instructions);
}
