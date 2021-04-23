# MySync
A simple, safe, and user-friendly two-way synchronization tool.

##Status
At this point, this repository is little more than a hello-world.

Below is the proposed operation:

##Commands
Command/Synopsys                  |  Description                         | Example
----------------------------------|--------------------------------------|------------------------------------
MySync Clone <Origin Directory>   | Clone a directory (Known as the origin) into the current directory (known as the local repository) | MySync Clone o:\myfiles\myphotos
MySync Sync [-i] [-push] [-pull] [-checksum] [-forcelock] | Synchronizes any files that have changed between the origin and local repository. Conflicts are ignored by default, and will keep appearing on each subsequent Sync, until they are resolved using one of the arguments: -i, -push, or -pull.<br/><br/>Options:<br/>-i&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;interactive conflict resolution, where you can choose to push (to origin), pull (from origin), or ignore.<br/><br/>-push&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;all conflicts resolved by pushing changes from the local repository to the origin. (not used with -i)<br/><br/>-pull&#9;all conflicts resolved by pulling changes from the origin to the local repository. (not used with -i)<br/><br/>-checksum&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;This will perform a deep comparison, where it compares not only by dates and size, but by file checksums.<br/><br/>-forcelock&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;This will remove the [origin]/.mysync/.lock file and create a new .lock file to process the sync. | MySync Sync
  
##Artifacts
Artifact                    | Description
----------------------------|-----------------------------------------------------
.mysync                     | This is a directory that exists in the root of both the origin and local repository<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* in the origin,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*.lock file - persists while one of the local repositories is synching files<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;YYYYMMDD_changelog.txt file(s) – to tell us what was changed in each sync<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(optionally) YYYYMMDD_conflicts.zip - if any conflicting files were overwritten by a file in a repository, they will be placed in this zip.<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(optionally) YYYYMMDD_retains.zip – if there is a file that matches a pattern in the .mysyncretain file, and it is being overwritten in the origin, then it will be placed in this zip. <br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;in the local repository,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;LastCommonSnapshot.xml – contains the snapshot that represents the last common set of files between the origin and repository.<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;YYYYMMDD_changelog.txt file(s) – to tell us what was changed in each sync<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(optionally) YYYYMMDD_conflicts.zip - if any conflicting files were overwritten by a file in the origin, they will be placed in this zip. 


Wow, nevermind.  This is ugly in Markdown... I'm going to need a better way to do this.
