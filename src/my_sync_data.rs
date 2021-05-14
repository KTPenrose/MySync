use std::time::Instant;

#[derive(Eq)]
pub enum SyncEntry {
    Origin(OriginInfo),
    File(FileInfo),
    Directory(DirInfo)
}

#[derive(Eq)]
pub struct OriginInfo {
    pub path: String,   
    pub children: Vec<SyncEntry>
}

impl PartialEq for OriginInfo {
    fn eq(&self, other:&Self)->bool {
        &self.path==&other.path &&
        vec_equals(&self.children,&other.children)
    }
}

#[derive(Eq)]
pub struct FileInfo {
    pub name: String,
    pub last_modified: Instant,
    pub created: Instant,
    pub size: u32
}

impl PartialEq for FileInfo {
    fn eq(&self, other:&Self)->bool {
        &self.name==&other.name &&
        &self.last_modified==&other.last_modified &&
        &self.created==&other.created &&
        &self.size==&other.size
    }
}

#[derive(Eq)]
pub struct DirInfo {
    pub name: String,    
    pub children: Vec<SyncEntry>
}

impl PartialEq for DirInfo {
    fn eq(&self, other:&Self)->bool {
        &self.name==&other.name &&
        vec_equals(&self.children,&other.children)
    }
}

impl PartialEq for SyncEntry {
    fn eq(&self, other:&Self)->bool {
        match &self {
            SyncEntry::Origin(origin_info)=> {
                match other {
                    SyncEntry::Origin(origin_info2) => {
                        origin_info==origin_info2
                    },
                    SyncEntry::File(_) => {
                        false
                    },
                    SyncEntry::Directory(_) => {
                        false
                    }
                }
            },
            SyncEntry::File(file_info)=> {
                match other {
                    SyncEntry::Origin(_) => {                        
                        false
                    },
                    SyncEntry::File(file_info2) => {
                        file_info==file_info2
                    },
                    SyncEntry::Directory(_) => {
                        false
                    }
                }
            },
            SyncEntry::Directory(dir_info)=> {
                match other {
                    SyncEntry::Origin(_) => {                        
                        false
                    },
                    SyncEntry::File(_) => {
                        false
                    },
                    SyncEntry::Directory(dir_info2) => {
                        dir_info==dir_info2
                    }
                }
            }
        }
    }
}

    // fn vec_compare(va: &[f64], vb: &[f64]) -> bool {
    //     (va.len() == vb.len()) &&  // zip stops at the shortest
    //      va.iter()
    //        .zip(vb)
    //        .all(|(a,b)| eq_with_nan_eq(*a,*b))
    // }
//}

fn vec_equals(v1:&Vec<SyncEntry>, v2:&Vec<SyncEntry>)->bool {
    v1.len()==v2.len() &&
     v1.iter().zip(v2).all(|(a, b)| a==b)
}



pub fn get_path(origin:&SyncEntry, to_item:&SyncEntry)->Option<String> {

    if let SyncEntry::Origin(origin_info) = origin {
        for child in &origin_info.children {
            let response = get_path_recursive(&to_item, &child, &origin_info.path);
            if let Option::Some(_) = response {
                return response;
            }               
        }
    }
    return Option::None;
}

fn get_path_recursive(to_item:&SyncEntry, current_item:&SyncEntry, parent_path:&String)->Option<String> {

    match current_item {
        SyncEntry::Origin(_)=> {
            //not possible
            return Option::None;
        },
        SyncEntry::File(file_info) => {
            if to_item==current_item {
                return Option::Some(path_combine(&parent_path,&file_info.name));
            }
            return Option::None; 
        },
        SyncEntry::Directory(dir_info) => {
            let new_parent_path:String = path_combine(&parent_path,&dir_info.name);
            if to_item==current_item {
                return Option::Some(new_parent_path);
            }
            for child in &dir_info.children {
                let response = get_path_recursive(&to_item, &child, &new_parent_path);
                if let Option::Some(_) = response {
                    return response;
                }               
            }
            return Option::None;
        }
    } 
}

fn path_combine(path_one:&String, path_two:&String)->String {
    let mut p1=path_one.to_string();
    let mut p2=path_two.to_string();
    if !p1.ends_with("/") {
        p1.push('/');
    }
    while p2.starts_with("/") {
        p2=p2.chars().skip(1).take(p2.len()-1).collect();
    }
    return format!("{}{}", p1,p2)
}


#[cfg(test)]
mod test {

    use crate::my_sync_data::{get_path, SyncEntry, OriginInfo, DirInfo};

    #[test]
    fn test_get_path() {

        let child3=SyncEntry::Directory( DirInfo {
            name: String::from("childthree"),
            children: vec![]
        });

        let origin=SyncEntry::Origin(OriginInfo{
            path:String::from("c:/this/is/a/test"),
            children: vec![SyncEntry::Directory( DirInfo {
                name: String::from("childone"),
                children: vec![SyncEntry::Directory( DirInfo {
                    name: String::from("childtwo"),
                    children: vec![child3]
                })]
            })]
        });

        if let SyncEntry::Origin(origin_info) = &origin {
            if let Option::Some(SyncEntry::Directory(dir_info1))= &origin_info.children.first() {
                if let Option::Some(SyncEntry::Directory(dir_info2))= &dir_info1.children.first() {
                    if let Option::Some(sync_entry)=dir_info2.children.first() {
                        let path_op=get_path(&origin, &sync_entry);

                        if let Option::Some(path) = path_op {
                            println!("path={}", path);
                            assert_eq!(path, String::from("c:/this/is/a/test/childone/childtwo/childthree"));
                            return;
                        }
                    }
                }
            }
        }

       assert!(false);
    }
}