use std::time::Instant;
use std::fs::read_dir;
use crate::my_sync_data::{SyncEntry, OriginInfo, FileInfo, DirInfo};

pub fn clone(origin: &String) {
    let mut children=Vec<sync_entry>::new();
    let mut rootData:SyncEntry = SyncEntry::Origin(OriginInfo {
        path:origin,
        children
    });
    
    clone_recursive(origin, origin)
}

fn clone_recursive(origin: &SyncEntry, nextItem: &SyncEntry) {
    read_dir()
}



