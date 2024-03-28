use std::fs;
use crate::utlis::readfilenameloop;
use crate::utlis::copy;
use serde_json::{Map, Value};
pub fn copy_main(name:&str,export:&str){
    let _ = copy(&format!("{}\\bin",name),&format!("{}\\bin",export));
    let binding = readfilenameloop(&format!("{}\\bin",name)).unwrap();
    let bin_names:Map<String,Value>=serde_json::from_value(binding).unwrap();
    for (_,values) in bin_names{
        let array=values.as_array().unwrap();
        for i in array{
            let i =i.to_string();
            let i: &str=&i[1..i.len()-1];
            let _ = copy(&format!("{name}\\bin\\{i}"),&format!("{export}\\bin\\{i}"));
        }
    }
    let _ = copy(&format!("{}\\DOMDocument.xml",name),&format!("{}\\DOMDocument.xml",export));
    let _ = fs::create_dir(&format!("{}\\LIBRARY",export));
    let _=fs::create_dir(&format!("{}\\LIBRARY\\exports",export));
    let _=fs::create_dir(&format!("{}\\LIBRARY\\movieclips",export));
    let _=fs::create_dir(&format!("{}\\LIBRARY\\resources",export));
    let _=fs::create_dir(&format!("{}\\LIBRARY\\shapes",export));
    let _ = copy(&format!("{}\\{}.xfl",name,name),&format!("{}\\{}.xfl",export,export));
}