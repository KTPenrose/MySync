mod my_sync_arg_parser;

fn main() {
    let args = std::env::args().skip(1);
    //let mut user_instructions = my_sync_library::UserInstructions::new();
    
    let user_instructions=my_sync_arg_parser::parse_args(args);


    println!("user_instructions=\n{}\n", user_instructions);   
}
