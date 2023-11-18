use std::collections::HashSet;

use indicatif::ProgressBar;
use regex::Regex;
use serde_json::{Value,json, Map};
use ping_lib::{
    debug::PingDebug,
    file::{read_string, copy},
};
pub fn read_export(file_path:&str)->Value{
    let file_string=read_string(file_path).unwrap();
    let re = Regex::new(r#"libraryItemName="(.*?)""#).unwrap();
    let mut list:Value=json!({
        "movieclips":[],
        "resources":[],
        "shapes":[],
    });
    // let mut progress_bar_max:u64=0;

    // for _ in re.captures_iter(&file_string) {
    //     progress_bar_max+=1;
    // }
    // let progress_bar = ProgressBar::new(progress_bar_max);
    for capture in re.captures_iter(&file_string) {
        let content = capture.get(1).unwrap().as_str().to_owned();
        let parts = content.split("/");
        let mut str_patrs:Vec<&str>=vec![];
        for part in parts{
            str_patrs.push(part);
        }
        let value=list.get_mut(str_patrs[0]).unwrap();
        let value_array=value.as_array_mut().unwrap();
        value_array.push(str_patrs[1].into());
        value_array.push(str_patrs[1].into());
        //progress_bar.inc(1);
    }
    //progress_bar.finish();
    //format!("task:{} finshed!",file_path).info();
    unique_vec(list)
    // let mut list:Map<String,Value>=serde_json::from_value(list).unwrap();

    // for(key,value)in list.clone(){
    //     match value {
    //         Value::Array(array)=>{
    //             let mut new_array:Vec<String>=vec![];
    //             for i in array{
    //                 let i =i.to_string();
    //                 let i=&i[1..i.len()-1];
    //                 new_array.push(i.to_string());
    //             }
    //             let unique_array: Vec<String> = new_array.into_iter().collect::<HashSet<String>>().into_iter().collect();
    //             let array=list.get_mut(&key).unwrap();
    //             let value_array=array.as_array_mut().unwrap();
    //             value_array.clear();
    //             for i in unique_array{
    //                 value_array.push(i.into());
    //             }
    //         },
    //         _=>{}
    //     }
    // };
    // serde_json::to_value(list).unwrap()
}
pub fn read_xml(export_folder:&str,src_folder:&str,type_name:&str,file_path:&str){
    let file_path=format!("{export_folder}\\LIBRARY\\{type_name}\\{file_path}");
    let mut file_string=String::new();
    match read_string(&file_path){
        Ok(content)=>file_string=content,
        Err(err)=>err.einfo(),
    };
    let re = Regex::new(r#"libraryItemName="(.*?)""#).unwrap();
    let mut list:Value=json!({
        "movieclips":[],
        "resources":[],
        "shapes":[],
    });
    for capture in re.captures_iter(&file_string) {
        let content = capture.get(1).unwrap().as_str().to_owned();
        let parts = content.split("/");
        let mut str_patrs:Vec<&str>=vec![];
        for part in parts{
            str_patrs.push(part);
        }
        let value=list.get_mut(str_patrs[0]).unwrap();
        let value_array=value.as_array_mut().unwrap();
        value_array.push(str_patrs[1].into());
    }
    let list=unique_vec(list);
    list.print();
    for movie_clip in list.get("movieclips").unwrap().as_array().unwrap(){
        let movieclip=movie_clip.to_string();
        let movieclip=&movieclip[1..movieclip.len()-1];
        let src_path=format!("{src_folder}\\LIBRARY\\movieclips\\{movieclip}.xml");
        let path=format!("{export_folder}\\LIBRARY\\movieclips\\{movieclip}.xml");
        movie_clip.info();
        let _ = copy(&src_path, &path);
        let _=read_xml(&export_folder,src_folder,"movieclips",&format!("{movieclip}.xml"));
    }
    for shapes in list.get("shapes").unwrap().as_array().unwrap(){
        let shapes=shapes.to_string();
        let shapes=&shapes[1..shapes.len()-1];
        let src_path=format!("{src_folder}\\LIBRARY\\shapes\\{shapes}.xml");
        let path=format!("{export_folder}\\LIBRARY\\shapes\\{shapes}.xml");
        shapes.info();
        let _ = copy(&src_path, &path);
        let _=read_xml(&export_folder,src_folder,"shapes",&format!("{shapes}.xml"));
    }
}

fn unique_vec(list:Value)->Value{
    let mut list:Map<String,Value>=serde_json::from_value(list).unwrap();

    for(key,value)in list.clone(){
        match value {
            Value::Array(array)=>{
                let mut new_array:Vec<String>=vec![];
                for i in array{
                    let i =i.to_string();
                    let i=&i[1..i.len()-1];
                    new_array.push(i.to_string());
                }
                let unique_array: Vec<String> = new_array.into_iter().collect::<HashSet<String>>().into_iter().collect();
                let array=list.get_mut(&key).unwrap();
                let value_array=array.as_array_mut().unwrap();
                value_array.clear();
                for i in unique_array{
                    value_array.push(i.into());
                }
            },
            _=>{}
        }
    };
    serde_json::to_value(list).unwrap()
}