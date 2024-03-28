use std::fs;
use crate::file::copy_main;
use indicatif::ProgressBar;
use crate::xml::{read_export, read_xml};
use crate::utlis::{readfilenameloop,copy};
pub fn mode_one(name:&str,line:&str){
    let exports_path=format!("{}\\LIBRARY\\exports\\",name);
    let file_name=line;
    let _ = fs::create_dir(file_name);
    let export_folder=line;
    let src_folder=name;

    let names:Vec<&str>=line.split(' ').collect();
    copy_main(name,&file_name);
    for name in names.clone(){
        let path=format!("{}{}.xml",&exports_path,name);
        let copy_path=format!("{}\\LIBRARY\\exports\\{name}.xml",export_folder);
        let _ = copy(&path, &copy_path);
        let values=read_export(&path);
        
        let movieclips=values.get("movieclips").unwrap().as_array().unwrap();
        let resources=values.get("resources").unwrap().as_array().unwrap();
        let shapes=values.get("shapes").unwrap().as_array().unwrap();
        for pngs in resources{
            let pngs=pngs.to_string();
            let pngs=&pngs[1..pngs.len()-1];
            let src_path=format!("{src_folder}\\LIBRARY\\resources\\{pngs}.png");
            let path=format!("{export_folder}\\LIBRARY\\resources\\{pngs}.png");
            let _ = copy(&src_path, &path);
        }
        for movieclip in movieclips{
            let movieclip=movieclip.to_string();
            let movieclip=&movieclip[1..movieclip.len()-1];
            let src_path=format!("{src_folder}\\LIBRARY\\movieclips\\{movieclip}.xml");
            let path=format!("{export_folder}\\LIBRARY\\movieclips\\{movieclip}.xml");
            let _ = copy(&src_path, &path);
            let _=read_xml(&export_folder,&src_folder,"movieclips",&format!("{movieclip}.xml"));
        }
        for shape in shapes{
            let shape=shape.to_string();
            let shape=&shape[1..shape.len()-1];
            let src_path=format!("{src_folder}\\LIBRARY\\shapes\\{shape}.xml");
            let path=format!("{export_folder}\\LIBRARY\\shapes\\{shape}.xml");
            let _ = copy(&src_path, &path);
        }
    }
}
pub fn mode_two(name:&str,line:&str){
    // "请输入生成的文件名(.fla或无)".info();
    // let mut file_name=String::new();
    // match io::stdin().read_line(&mut file_name){
    //     Ok(_)=>{},
    //     Err(err)=>{err.einfo();exit(0x0100)}
    // };

    let export_folder=line;
    let src_folder=name;
    let _ = fs::create_dir(export_folder);
    copy_main(name,export_folder);
    let key_words:Vec<&str>=line.split(' ').collect();
    
    let exports_path=format!("{}\\LIBRARY\\exports\\",name);
    let files=readfilenameloop(&exports_path).unwrap();

    let names= files.get(&exports_path).unwrap().as_array().unwrap();
    let mut progress_bar_max:u64=0;


    let mut xml_names:Vec<String>=vec![];
    for word in key_words{
        for name in names{
            let name=name.to_string();
            let name=&name[1..name.len()-1];
            if name.contains(&word){xml_names.push(name.to_owned());
               progress_bar_max+=1;
            }
        }
        }
    let progress_bar = ProgressBar::new(progress_bar_max);
    for name in xml_names{
        let path=format!("{}{}",&exports_path,name);
        let copy_path=format!("{}\\LIBRARY\\exports\\{name}",export_folder);
        let _ = copy(&path, &copy_path);
        let values=read_export(&path);
        
        let movieclips=values.get("movieclips").unwrap().as_array().unwrap();
        let resources=values.get("resources").unwrap().as_array().unwrap();
        let shapes=values.get("shapes").unwrap().as_array().unwrap();
        for pngs in resources{
            let pngs=pngs.to_string();
            let pngs=&pngs[1..pngs.len()-1];
            let src_path=format!("{src_folder}\\LIBRARY\\resources\\{pngs}.png");
            let path=format!("{export_folder}\\LIBRARY\\resources\\{pngs}.png");
            let _ = copy(&src_path, &path);
        }
        for movieclip in movieclips{
            let movieclip=movieclip.to_string();
            let movieclip=&movieclip[1..movieclip.len()-1];
            let src_path=format!("{src_folder}\\LIBRARY\\movieclips\\{movieclip}.xml");
            let path=format!("{export_folder}\\LIBRARY\\movieclips\\{movieclip}.xml");
            let _ = copy(&src_path, &path);
            let _=read_xml(export_folder,&src_folder,"movieclips",&format!("{movieclip}.xml"));
        }
        for shape in shapes{
            let shape=shape.to_string();
            let shape=&shape[1..shape.len()-1];
            let src_path=format!("{src_folder}\\LIBRARY\\shapes\\{shape}.xml");
            let path=format!("{export_folder}\\LIBRARY\\shapes\\{shape}.xml");
            let _ = copy(&src_path, &path);
        }
       progress_bar.inc(1);
    }
    progress_bar.finish();
    
}