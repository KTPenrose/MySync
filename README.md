# MySync
A simple, safe, and user-friendly two-way synchronization tool.

## Status
At this point, this repository is little more than a hello-world.

## Proposed Operation

### Commands
Command/Synopsys                  |  Description                         | Example
----------------------------------|--------------------------------------|------------------------------------
MySync Clone <Origin Directory>   | Clone a directory (Known as the origin) into the current directory (known as the local repository) | MySync Clone o:\myfiles\myphotos
  MySync Sync [-i] [-push] [-pull] [-checksum] [-forcelock] | Synchronizes any files that have changed between the origin and local repository. Conflicts are ignored by default, and will keep appearing on each subsequent Sync, until they are resolved using one of the arguments: -i, -push, or -pull. | MySync Sync
&nbsp; |`Options:`| &nbsp;
&nbsp; |-i&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;interactive conflict resolution, where you can choose<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;to push (to origin), pull (from origin), or ignore|
&nbsp; |-push&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;all conflicts resolved by pushing changes from the local<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;repository to the origin. (not used with -i)|
&nbsp; |-pull&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;all conflicts resolved by pulling changes from the origin<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;to the local repository. (not used with -i)|  
&nbsp; |-checksum&nbsp;&nbsp;&nbsp;This will perform a deep comparison, where it compares<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;not only by dates and size, but by file checksums.|   
&nbsp; |-forcelock&nbsp;&nbsp;&nbsp;This will remove the [origin]/.mysync/.lock file and create<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;a new .lock file to process the sync. | 
  
### Artifacts

Artifact                    | Description
----------------------------|-----------------------------------------------------
.mysync                     | This is a directory that exists in the root of both the origin and local repository<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* in the origin,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* .lock file - persists while one of the local repositories is synching files<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* YYYYMMDD_changelog.txt file(s) – to tell us what was changed in each sync<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* (optionally) YYYYMMDD_conflicts.zip - if any conflicting files were overwritten by a file in a repository, <br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; they will be placed in this zip.<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* (optionally) YYYYMMDD_retains.zip – if there is a file that matches a pattern in the .mysyncRetain<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;file, and it is being overwritten in the origin, then it will be placed in this zip. <br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* in the local repository,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* LastCommonSnapshot.xml – contains the snapshot that represents the last common set of files<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; between the origin and repository.<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* YYYYMMDD_changelog.txt file(s) – to tell us what was changed in each sync<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;* (optionally) YYYYMMDD_conflicts.zip - if any conflicting files were overwritten by a file in the origin,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; they will be placed in this zip. 
.mysyncIgnore               | Placed in the root of the origin, this works like a .gitignore
.mysyncRetain               | Placed in the root of the origin, this tells the MySync process which files to retain (so that past versions of the file are preserved in the YYYYMMDD_retains.zip files).

### Snapshots
![Snapshots](./docs/Snapshots.png)

### Action Table

Local Repository | Origin | Action
-----------------|--------|--------
No Change | No Change | None
No Change | Deleted | Pull (delete from local repository)
No Change | (No File) | Created Pull (copy on local repository)
No Change | Modified | Pull (copy to local repository)
Deleted | No Change | Push (delete from origin)
Deleted | Deleted | None
Deleted | Modified | Conflict
Deleted | Created | Not Possible
Created | Created | Conflict (if different) or None (if same)
Created | No Change (No File) | Push (copy to origin)
Created | Modified | Not Possible
Created | Deleted | Not Possible
Modified | No Change | Push (copy to origin)
Modified | Modified | Conflict (if different) or None (if same)
Modified | Deleted | Conflict
Modified | Created | Not Possible
 
