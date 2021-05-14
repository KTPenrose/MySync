use std::fmt;

pub enum ActionTypes {
    None=0,
    Clone=1,
    Sync=2,
    Relink=3,
}

impl ActionTypes {
    pub fn is_none(&self) -> bool {
        match self {
            ActionTypes::None => true,
            _ => false
        }
    }
}

impl fmt::Display for ActionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ActionTypes::None=> { "None" },
            ActionTypes::Clone=> { "Clone" },
            ActionTypes::Sync=> { "Sync" },
            ActionTypes::Relink=> { "Relink" }
        })
    }
}
pub enum SyncStrategy {
    Sync=0,
    Push=1,
    Pull=2
}

impl fmt::Display for SyncStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            SyncStrategy::Sync=> { "Sync" },
            SyncStrategy::Push=> { "Push" },
            SyncStrategy::Pull=> { "Pull" }
        })
    }
}

pub struct UserInstructions {
    pub is_help: bool,
    pub is_full_help: bool,
    pub action:ActionTypes,
    pub is_interactive:bool,
    pub sync_strategy:SyncStrategy,
    pub do_checksum:bool,
    pub force_lock:bool,
    pub origin:Option<String>,
    pub error_messages: Vec<String>
}

impl UserInstructions {
    pub fn new()->UserInstructions {
        UserInstructions{ is_help:false, is_full_help:false, action:ActionTypes::None, is_interactive:false, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::None,
            error_messages: Vec::new() }
    }
}

impl fmt::Display for UserInstructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut error_messages: String = "".to_owned();
        for error_msg in &self.error_messages {
            error_messages.push_str(&error_msg);
            error_messages.push_str("\n");            
        }

        write!(f, "is_help: {}\nis_full_help: {}\naction:{}\nsync_strategy:{}\nis_interactive:{}\ndo_checksum:{}\nforce_lock:{}\norigin:{}\nerrors:{}", 
        self.is_help, self.is_full_help, self.action, self.sync_strategy, self.is_interactive, self.do_checksum, self.force_lock, 
        self.origin.as_ref().unwrap_or(&"(unspecified)".to_string()),&error_messages)
    }
}

pub fn parse_args(args:impl Iterator<Item=String>)->UserInstructions {
    let mut user_instructions = UserInstructions::new();
      

    for mut arg in args {
        arg=arg.to_lowercase();
        if arg=="help" || arg=="-h" {
            user_instructions.is_help=true;
            user_instructions.is_full_help=true;
        } else if arg=="clone" {
            if !user_instructions.action.is_none() {
                user_instructions.error_messages.push(format!("Two actions defined: '{}' and 'clone'", user_instructions.action));
            } else {
                user_instructions.action=ActionTypes::Clone;
            }
        } else if arg=="sync" {
            if !user_instructions.action.is_none() {
                user_instructions.error_messages.push(format!("Two actions defined: '{}' and 'sync'", user_instructions.action));
            } else {
                user_instructions.action=ActionTypes::Sync;
            }
        } else if arg=="relink" {
            if !user_instructions.action.is_none() {
                user_instructions.error_messages.push(format!("Two actions defined: '{}' and 'relink'", user_instructions.action));
            } else {
                user_instructions.action=ActionTypes::Relink;
            }
        } else if arg=="-i" {
            if !matches!(user_instructions.sync_strategy,SyncStrategy::Sync) {
                user_instructions.error_messages.push(format!("Can't use the -i flag with sync strategy '{}'", user_instructions.sync_strategy));
            } else if !matches!(user_instructions.action,ActionTypes::Sync) {
                user_instructions.error_messages.push(format!("Can't use the -i flag with action '{}'", user_instructions.action));
            } else {
                user_instructions.is_interactive=true;
            }
        } else if arg=="-push" {
            if !matches!(user_instructions.action,ActionTypes::Sync) {
                user_instructions.error_messages.push(format!("Can't use the -pull flag with action '{}'", user_instructions.action));
            } else if user_instructions.is_interactive {
                user_instructions.error_messages.push(format!("Can't use the -i flag with sync strategy 'Push'"));
            } else if matches!(user_instructions.sync_strategy,SyncStrategy::Pull) {
                user_instructions.error_messages.push(format!("Can't use the -push flag at the same time as the -pull flag"));
            } else {
                user_instructions.sync_strategy=SyncStrategy::Push;
            }
        } else if arg=="-pull" {
            if !matches!(user_instructions.action,ActionTypes::Sync) {
                user_instructions.error_messages.push(format!("Can't use the -pull flag with action '{}'", user_instructions.action));
            } else if user_instructions.is_interactive {
                user_instructions.error_messages.push(format!("Can't use the -i flag with sync strategy 'Pull'"));
            } else if matches!(user_instructions.sync_strategy,SyncStrategy::Push) {
                user_instructions.error_messages.push(format!("Can't use the -pull flag at the same time as the -push flag"));
            } else {
                user_instructions.sync_strategy=SyncStrategy::Pull;
            }
        } else if arg=="-checksum" {
            if !matches!(user_instructions.action, ActionTypes::Sync) {
                user_instructions.error_messages.push(format!("Can't use the -checksum flag with action '{}'", user_instructions.action));
            } else {
                user_instructions.do_checksum=true;
            }
        } else if arg=="-forcelock" {
           user_instructions.force_lock=true;
        } else {
            //assume this is an origin!!
            if user_instructions.origin.is_some() {
                let origin=user_instructions.origin.clone();
                user_instructions.error_messages.push(format!("Is '{}' an origin? I was already using '{}' as the origin...", arg, origin.unwrap()));
            }
            else if !matches!(user_instructions.action, ActionTypes::Clone) &&
                    !matches!(user_instructions.action, ActionTypes::Relink) {
                user_instructions.error_messages.push(format!("Unexpected argument: {}", arg));
            } else {
                user_instructions.origin=Option::Some(arg);
            }
        }
    }
    if user_instructions.error_messages.len()>0 {
        user_instructions.is_help=true;
    }

    if user_instructions.action.is_none() {
        user_instructions.is_help=true;
        if !user_instructions.is_full_help {
            if user_instructions.error_messages.len()==0 {
                user_instructions.is_full_help=true;  
            }
            user_instructions.error_messages.push(format!("No action specified."));
        }
    } else if matches!(user_instructions.action,ActionTypes::Clone) {
        if user_instructions.origin.is_none() {
            user_instructions.is_help=true;
            user_instructions.error_messages.push(format!("No origin was specified."));
        }
    } else if matches!(user_instructions.action,ActionTypes::Relink) {
        if user_instructions.origin.is_none() {
            user_instructions.is_help=true;
            user_instructions.error_messages.push(format!("No origin was specified."));
        }
    }

    return user_instructions;
}


#[cfg(test)]
mod test {

    //use std::array::IntoIter;
    //use my_sync::my_sync_arg_parser::*;
    use crate::my_sync_arg_parser::UserInstructions;
    use crate::my_sync_arg_parser::ActionTypes;
    use crate::my_sync_arg_parser::SyncStrategy;

    fn matchui(ui:UserInstructions, myref:UserInstructions) {
        assert!(ui.is_help==myref.is_help);
        assert!(ui.action as u8==myref.action as u8);
        assert!(ui.is_interactive==myref.is_interactive);
        assert!(ui.sync_strategy as u8==myref.sync_strategy as u8);
        assert!(ui.do_checksum==myref.do_checksum);
        assert!(ui.force_lock==myref.force_lock);
        if ui.origin.is_some() && myref.origin.is_some() {
            assert!(ui.origin.unwrap()==myref.origin.unwrap());
        } else if ui.origin.is_none() && myref.origin.is_none() {
            //were good!
        } else {
            assert!(false, "origin's don't match!");
        }
        assert!(ui.error_messages.len()==myref.error_messages.len());
        for i in 0..ui.error_messages.len() {
            assert_eq!(&ui.error_messages[i],&myref.error_messages[i])
        }       
    }

    //mysync clone c:\mydir
    #[test]
    fn test_clone_with_origin() {
        let args=std::array::IntoIter::new(["clone".to_string(),"c:\\mydir".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Clone, is_interactive:false, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::Some("c:\\mydir".to_string()),
            error_messages: Vec::new() });
    }

    //mysync sync -i
    #[test]
    fn test_sync_interactive() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-i".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:true, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::None,
            error_messages: Vec::new() });
    }

    //mysync sync -pull
    #[test]
    fn test_sync_pull() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-pull".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:false, 
            sync_strategy:SyncStrategy::Pull, do_checksum:false, force_lock:false, origin: Option::None,
            error_messages: Vec::new() });
    }

    //mysync sync -pull -checksum
    #[test]
    fn test_sync_pull_checksum() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-pull".to_string(),"-checksum".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:false, 
            sync_strategy:SyncStrategy::Pull, do_checksum:true, force_lock:false, origin: Option::None,
            error_messages: Vec::new() });
    }

    //mysync sync -pull -checksum -forcelock
    #[test]
    fn test_sync_pull_checksum_forcelock() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-pull".to_string(),"-checksum".to_string(), "-forcelock".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:false, 
            sync_strategy:SyncStrategy::Pull, do_checksum:true, force_lock:true, origin: Option::None,
            error_messages: Vec::new() });
    }

       //mysync sync -push
       #[test]
       fn test_sync_push() {
           let args=std::array::IntoIter::new(["sync".to_string(),"-push".to_string()]);
           let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
           matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:false, 
               sync_strategy:SyncStrategy::Push, do_checksum:false, force_lock:false, origin: Option::None,
               error_messages: Vec::new() });
       }

       //mysync sync -pull -checksum
    #[test]
    fn test_sync_push_checksum() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-push".to_string(),"-checksum".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:false, 
            sync_strategy:SyncStrategy::Push, do_checksum:true, force_lock:false, origin: Option::None,
            error_messages: Vec::new() });
    }

    //mysync sync -push -checksum -forcelock
    #[test]
    fn test_sync_push_checksum_forcelock() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-push".to_string(),"-checksum".to_string(), "-forcelock".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Sync, is_interactive:false, 
            sync_strategy:SyncStrategy::Push, do_checksum:true, force_lock:true, origin: Option::None,
            error_messages: Vec::new() });
    }

        //mysync clone -i
        #[test]
        fn test_clone_no_origin_interactive() {
            let args=std::array::IntoIter::new(["clone".to_string(),"-i".to_string()]);
            let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
            assert!(user_instructions.is_help);
            assert_eq!(user_instructions.error_messages.len(), 2);  
            assert_eq!("Can't use the -i flag with action 'Clone'", user_instructions.error_messages[0]);
            assert_eq!("No origin was specified.", user_instructions.error_messages[1]);
        }

    //mysync clone c:\mydir -i
    #[test]
    fn test_clone_interactive() {
        let args=std::array::IntoIter::new(["clone".to_string(),"c:\\mydir".to_string(),"-i".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        assert!(user_instructions.is_help);
        assert_eq!(user_instructions.error_messages.len(), 1); 
        assert_eq!("Can't use the -i flag with action 'Clone'", user_instructions.error_messages[0]);       
    }

    //mysync sync c:\mydir
    #[test]
    fn test_sync_with_origin() {
        let args=std::array::IntoIter::new(["sync".to_string(),"c:\\mydir".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        assert!(user_instructions.is_help);
        assert_eq!(user_instructions.error_messages.len(), 1); 
        assert_eq!("Unexpected argument: c:\\mydir", user_instructions.error_messages[0]);       
    }

    //mysync sync -i -push
    #[test]
    fn test_sync_interactive_push() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-i".to_string(),"-push".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        assert!(user_instructions.is_help);
        assert_eq!(user_instructions.error_messages.len(), 1); 
        assert_eq!("Can't use the -i flag with sync strategy 'Push'", user_instructions.error_messages[0]);       
    }

    //mysync sync -push -i
    #[test]
    fn test_sync_push_interactive() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-push".to_string(),"-i".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        assert!(user_instructions.is_help);
        assert_eq!(user_instructions.error_messages.len(), 1); 
        assert_eq!("Can't use the -i flag with sync strategy 'Push'", user_instructions.error_messages[0]);       
    }

    
    //mysync sync -pull -i
    #[test]
    fn test_sync_pull_interactive() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-pull".to_string(),"-i".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        assert!(user_instructions.is_help);
        // for msg in &user_instructions.error_messages {
        //     println!("error: {}", &msg);
        // }
        assert_eq!(user_instructions.error_messages.len(), 1); 
        assert_eq!("Can't use the -i flag with sync strategy 'Pull'", user_instructions.error_messages[0]);       
    }

    //mysync sync -pull -i
    #[test]
    fn test_sync_interactive_pull() {
        let args=std::array::IntoIter::new(["sync".to_string(),"-i".to_string(),"-pull".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
        
        let mut error_messages = Vec::new();
        error_messages.push("Can't use the -i flag with sync strategy 'Pull'".to_string());
        matchui (user_instructions, UserInstructions { is_help:true, is_full_help:false, action:ActionTypes::Sync, is_interactive:true, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::None,
            error_messages });
    }

    //mysync clone -i "my fancy directory" -pull garbage
    #[test]
    fn test_clone_interactive_pull_garbage() {
        let args=std::array::IntoIter::new(["clone".to_string(),"-i".to_string(),"my fancy directory".to_string(),"-pull".to_string(),"garbage".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);

        let mut error_messages = Vec::new();
        error_messages.push("Can't use the -i flag with action 'Clone'".to_string());
        error_messages.push("Can't use the -pull flag with action 'Clone'".to_string()); 
        error_messages.push("Is 'garbage' an origin? I was already using 'my fancy directory' as the origin...".to_string()); 
        matchui (user_instructions, UserInstructions { is_help:true, is_full_help:false, action:ActionTypes::Clone, is_interactive:false, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::Some("my fancy directory".to_string()),
            error_messages });      
    }

    //mysync relink
    #[test]
    fn test_relink() {
        let args=std::array::IntoIter::new(["relink".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);

        let mut error_messages = Vec::new();
        error_messages.push("No origin was specified.".to_string());
        matchui (user_instructions, UserInstructions { is_help:true, is_full_help:false, action:ActionTypes::Relink, is_interactive:false, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::None,
            error_messages });      
    }

    //mysync relink "my fancy directory"
    #[test]
    fn test_relink_to_directory() {
        let args=std::array::IntoIter::new(["relink".to_string(),"my fancy directory".to_string()]);
        let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);

        let error_messages = Vec::new();
        matchui (user_instructions, UserInstructions { is_help:false, is_full_help:false, action:ActionTypes::Relink, is_interactive:false, 
            sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::Some("my fancy directory".to_string()),
            error_messages });      
    }

        //mysync help"
        #[test]
        fn test_help() {
            let args=std::array::IntoIter::new(["help".to_string()]);
            let user_instructions:UserInstructions=crate::my_sync_arg_parser::parse_args(args);
            for msg in &user_instructions.error_messages {
                println!("error: {}", &msg);
            }
    
            let error_messages = Vec::new();
            matchui (user_instructions, UserInstructions { is_help:true, is_full_help:true, action:ActionTypes::None, is_interactive:false, 
                sync_strategy:SyncStrategy::Sync, do_checksum:false, force_lock:false, origin: Option::None,
                error_messages });      
        }
}