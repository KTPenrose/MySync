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

pub fn display_help(show_full_help:bool) {
    if show_full_help {
        println!();
        println!("MySync is a simple, safe, and user-friendly two-way synchronization tool.");
        println!();
        println!("  * Simple = a few, well explained, and obvious options");
        println!("  * Safe = conflicting files are never overwritten by default, and never overwritten without being backed up.");
        println!("  * User-friendly = if your not sure, come ask.  https://mysync.app");
        println!();
        println!("MySync can be used to back up and synchronize files to a centralized file location.  It excels at maintaining a");
        println!("synchronized directory structure across multiple machines, without being brittle about simultaneous change.  It");
        println!("does not, however, attempt to synchronize simultaneous changes to a single file, and refers to this as a conflict.");
        println!("It also excels at maintaining a full backup of a directory structure in second location.  This should work on all");
        println!("systems and filesystems, though some care will need to be made when naming files and directories, if you are attempting");
        println!("to cross filesystem types (such as between NTFS and EXT4 or APFS).  I apologize ahead of time that the examples listed");
        println!("below are for Windows(TM) systems.  This was done only because file paths are more obviously file paths on Windows.");
        println!();
        println!("To understand how it works, you must first understand some terminology:");
        println!();
        println!(" ORIGIN - This is some centrally-accessible directory that all machines must sync to/from.  This could be one of many ");
        println!("          things, for instance (1) A network directory, (2) a mounted cloud drive such as OneDrive, iCloud, Google Drive,");
        println!("          or any other cloud provider which provides client software to make their services appear as a local drive,");
        println!("          (3) an (s)ftp or ssh that has been mounted as a local filesystem, (4) probably other scenarios.");
        println!();
        println!(" LOCAL REPOSITORY - This will serve as your local working copy of the ORIGIN.  It is created by opening a command shell to");
        println!("                    the location where you would like to have your LOCAL REPOSITORY, and running a 'clone' command, for");
        println!("                    example, 'MySync clone x:\\some\\network\\location\\myfiles'.  In this example, if you had your command");
        println!("                    shell open to c:\\mylocalfiles while running the example command, then it would create a LOCAL");
        println!("                    REPOSITORY at c:\\mylocalfiles\\myfiles.");
        println!();
        println!(" CLONE - This is the action of creating a LOCAL REPOSITORY from a central ORIGIN.  You would typically only run the CLONE");
        println!("         command once for a given LOCAL REPOSITORY.  In fact MySync will prevent you from doing it a second time, because");
        println!("         it would have the effect of possibly overwriting your local files.  You can, of course, get MySync to CLONE again");
        println!("         by completely deleting the LOCAL REPOSITORY and starting fresh.");
        println!();
        println!(" SYNC - This is the action of synchronizing changes between the LOCAL REPOSITORY and central ORIGIN.  Most commonly, and");
        println!("        the default behavior, one would allow two-way changes to occur. This behavior could be overridden, to force ");
        println!("        MySync to either PUSH all changes to the ORIGIN, or to PULL all changes to the LOCAL REPOSITORY.");
        println!("        Once you have a LOCAL REPOSITORY, and you want to synchronize changes, you would open a command prompt to this");
        println!("        directory and execute the command 'MySync sync'.  You will never need to re-specify the path to the ORIGIN, because");
        println!("        MySync already knows this path.");
        println!();
        println!(" RELINK - This is the action of re-pointing a LOCAL REPOSITORY to its ORIGIN in cases where the file path of the ORIGIN");
        println!("          has changed.  This command should be executed with care.");
        println!();
        println!(" .mysync - This is the a directory maintained by MySync that holds everything necessary to ensure files are properly");
        println!("           synchronized across directories.  This directory will exist on the ORIGIN and all LOCAL REPOSITOR(Y|IES),");
        println!("           but each instance of the .mysync directory will contain different files, depending on its role as an ORIGIN");
        println!("           or LOCAL REPOSITORY.  As a general rule, you should never delete or modify this directory, or anything in it.");
        println!("           There are, however, times when you may need to retrieve something from it.  For instance, whenever a conflict");
        println!("           occurs, and the user chooses to overwrite a file (by PUSH, PULL, or INTERACTIVELY), a backup copy of the");
        println!("           overwritten file will be placed into a special location within this directory.");
        println!();
        println!("MySync works superficially similar to GIT... It produces a subdirectory (named .mysync) which manages all of the details.");
        println!();
        println!("           On the ORIGIN, the .mysync directory will contain:");
        println!();
        println!("              * .lock - this file is created by a running instance of MySync that is either performing either a CLONE");
        println!("                        or SYNC operation.  This is done to prevent a second copy of MySync from interacting with the");
        println!("                        first copy in an undefined way.  This file should be deleted automatically when MySync completes");
        println!("                        its task, although there is a '-forcelock' option that can force an instance of MySync to claim");
        println!("                        the lock, in case it is a leftover from another MySync that ended abruptly.");
        println!();
        println!("              * YYYYMMDD_changelog.txt - One of these files is created every time a file was changed in the ORIGIN as the");
        println!("                                         result of a SYNC operation.");
        println!();
        println!("              * YYYYMMDD_conflicts.txt - One of these files is created if a SYNC operation encounters one or more");
        println!("                                         conflicts."); 
        println!();
        println!("              * YYYYMMDD_retains.zip - One of these files is created if a SYNC operation encounters a situation where");
        println!("                                       (1) one or more files are overwritten in the ORIGIN whose filename(s) match(es)");
        println!("                                           one of the patterns in the .mysyncRetains file."); 
        println!("                                       (2) A conflict was resolved by overwriting a file in the ORIGIN."); 
        println!();
        println!("           On the LOCAL REPOSITORY, the .mysync directory will contain:");
        println!();
        println!("              * LastCommonSnapshot.xml - contains the snapshot that represents the last common set of files");
        println!("                                         between the ORIGIN and LOCAL REPOSITORY.  It also contains the path to the");
        println!("                                         ORIGIN.");
        println!();
        println!("              * YYYYMMDD_changelog.txt - One of these files is created every time a file was changed in the LOCAL");
        println!("                                         REPOSITORY as a result of a SYNC operation.");
        println!();
        println!("              * YYYYMMDD_retains.zip - One of these files is created if a SYNC operation encounters a situation where");
        println!("                                       (1) one or more files are overwritten in the LOCAL REPOSITORY whose filename(s)");
        println!("                                           match(es) one of the patterns in the .mysyncRetains file."); 
        println!("                                       (2) A conflict was resolved by overwriting a file in the LOCAL REPOSITORY."); 
    } else {
        println!();
        println!("For more complete help, run 'MySync help'"); 
    }
    println!();
    println!("SYNOPSIS");
    println!();
    println!("   MySync help");
    println!("   MySync clone origin_directory  [-forcelock]");
    println!("   MySync Sync [-i] [-push] [-pull] [-checksum] [-forcelock]");
    println!("   MySync relink origin_directory");
    println!();
    println!("OPTIONS");
    println!();
    println!(" -i      Interactive conflict resolution, where you can chose to push (to ORIGIN), pull (to LOCAL REPOSITORY),");
    println!("         or ignore the conflict.  Ignored conflicts with reappear as conflicts in subsequent SYNC operations.");
    println!("         (Note: Used with sync action, this option can't be used with -push or -pull)");
    println!();
    println!(" -push   All conflicts resolved by pushing changes from the LOCAL REPOSITORY to the ORIGIN.");
    println!("         (Note: Used with sync action, this option can't be used with -i or -pull)");
    println!("");
    println!(" -pull   All conflicts resolved by pulling changes from the ORIGIN to the LOCAl REPOSITORY.");
    println!("         (Note: Used with sync action, this option can't be used with -i or -push)");
    println!();
    println!(" -checksum  This will perform a deep comparison, where it compares not only by dates and file sizes, but also");
    println!("            file checksum. (Note: Used with sync action)");
    println!();
    println!(" -forcelock  This will remove the [ORIGIN]/.mysync/.lock file and create a new .lock file to process the");
    println!("              the sync or clone action.");
    println!();
    println!(" -h      Synonymous with 'MySync help'.  This option will produce the full help text.");
}