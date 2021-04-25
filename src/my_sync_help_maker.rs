use crate::my_sync_arg_parser::UserInstructions;


pub fn display_any_problems(usr_inst: &UserInstructions) {
    if usr_inst.error_messages.len()>0 {
        if usr_inst.error_messages.len()>1 {
            println!("MySync encountered {} problems.", usr_inst.error_messages.len());
            let mut i=1;
            for msg in &usr_inst.error_messages {
                println!("Problem {}: {}", i, msg);
                i+=1;
            }
        } else {
            for msg in &usr_inst.error_messages {
                println!("MySync encountered a problem: {}", msg);
            }
        }
    }
}

pub fn display_help() {
    println!();
    println!("Action                           | Description                                      | Example");
    println!("=================================+==================================================+==================================");
    println!("MySync Clone <Origin Directory>  | Clone a directory (Known as the origin)          | MySync Clone o:\\myfiles\\myphotos");
    println!("        [-forcelock]             |                                                  |");
    println!("                                 |                                                  |");
    println!("MySync Sync [-i] [-push] [-pull] | Synchronizes any files that have changed between | MySync Sync");
    println!("        [-checksum] [-forcelock] | the origin and local repository.  Conflicts are  |");
    println!("                                 | ignored by default, and will keep appearing on   |");
    println!("                                 | each subsequent Sync, until they are resolved    |");
    println!("                                 | using one of the arguments: -i, -push, or -pull. |");
    println!("                                 |                                                  |");
    println!("                                 | Options:                                         |");
    println!("                                 | -i      interactive conflict resolution, where   |");
    println!("                                 |         you can chose to push (to origin), pull  |");
    println!("                                 |         (from origin), or ignore                 |");
    println!("                                 |                                                  |");
    println!("                                 | -push   all conflicts resolved by pushing changes|");
    println!("                                 |         from the local repository to the origin. |");
    println!("                                 |         (Note: this option can't be used with -i)|");
    println!("                                 |                                                  |");
    println!("                                 | -pull   all conflicts resolved by pulling changes|");
    println!("                                 |         from the origin to the local repository. |");
    println!("                                 |         (Note: this option can't be used with -i)|");
    println!("                                 |                                                  |");
    println!("                                 | -checksum  This will perform a deep comparison,  |");
    println!("                                 |            where it compares not only by dates   |");
    println!("                                 |            and size, but by file checksum.       |");
    println!("                                 |                                                  |");
    println!("                                 | -forcelock  This will remove the                 |");
    println!("                                 |             [origin]/.mysync/.lock file and      |");
    println!("                                 |             create a new .loc file to process    |");
    println!("                                 |             the sync or clone.                   |");
    println!("=================================+==================================================+==================================");
}