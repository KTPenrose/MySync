mod my_sync_arg_parser;
mod my_sync_help_maker;

fn main() {
    let usr_inst = my_sync_arg_parser::parse_args(std::env::args().skip(1));

    my_sync_help_maker::display_any_problems(&usr_inst);

    if usr_inst.is_help {
        my_sync_help_maker::display_help();
    } else {
        match usr_inst.action {
            my_sync_arg_parser::ActionTypes::Clone => {
                //do clone!
            },
            my_sync_arg_parser::ActionTypes::Sync => {
                //do sync!
            },
            my_sync_arg_parser::ActionTypes::None => {
                //not actually possible, but no harm no foul
                my_sync_help_maker::display_help();
            },
        }
    }

    println!("user_instructions=\n{}\n", usr_inst);
}
