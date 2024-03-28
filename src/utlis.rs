use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use anyhow::Result;
use colored::Colorize;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use zip::ZipArchive;
use zip::{ZipWriter, write::FileOptions};
use std::fmt::Debug;
use chrono::{Local, DateTime};

pub trait PingDebug{
    fn print(&self)where Self: Debug{
        println!("{:?}",&self);
    }
    fn info(&self)where Self: Debug{
        let utc: DateTime<Local> = Local::now(); 
        let info_str="INFO".green();
        println!("{} {} {:?}",utc,info_str,&self);
    }
    fn einfo(&self)where Self: Debug{
        let utc: DateTime<Local> = Local::now(); 
        let info_str="ERROR".red();
        println!("{} {} {:?}",utc,info_str,&self);
    }
}
impl<U> PingDebug for U{
    fn print(&self)where Self: Debug{
        println!("{:?}",&self);
    }
    fn info(&self)where Self: Debug{
        let utc: DateTime<Local> = Local::now(); 
        let info_str="INFO".green();
        println!("{} {} {:?}",utc,info_str,&self);
    }
    fn einfo(&self)where Self: Debug{
        let utc: DateTime<Local> = Local::now(); 
        let info_str="ERROR".red();
        println!("{} {} {:?}",utc,info_str,&self);
    }
}
pub fn info<S>(message:S)
where
S: Into<String>+ std::fmt::Debug,
{
    let utc: DateTime<Local> = Local::now(); 
    let info_str="INFO".green();
    println!("{} {} {:?}",utc,info_str,message);
}

pub fn read_string(path: &str) -> Result<String> {
    let mut file=File::open(path)?;
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    Ok(content)
}

pub fn copy(src: &str, dest: &str) ->Result<()> {
    let src_path=Path::new(src);
    let dest_path=Path::new(dest);
    if src_path.is_dir(){
        let _ = fs::create_dir(dest_path);
        let _ = fs::copy(src_path, dest_path);
    }else{
        let src_file = File::open(src)?;
        let dest_file = File::create(dest)?;
    
        let mut reader = BufReader::new(src_file);
        let mut writer = BufWriter::new(dest_file);
    
        io::copy(&mut reader, &mut writer)?;
    }
    Ok(())
}


pub fn write(path: &str, data: &[u8]) -> Result<()>{
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn unzip(zip_path: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 打开zip文件
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let output_dir_path=Path::new(output_dir);
    if output_dir_path.exists(){
        println!("output path is already exists");
        return Ok(());
    }
    // 遍历zip文件中的文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let output_path = format!("{}/{}", output_dir, file.name());
        
        // 创建输出文件
        if (file.name()).ends_with('/') {
            std::fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent_dir) = std::path::Path::new(&output_path).parent() {
                if !parent_dir.exists() {
                    std::fs::create_dir_all(parent_dir)?;
                }
            }
            let mut output_file = File::create(&output_path)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}

pub fn readfilenameloop(path:&str)
->Result<Value>
{
    let mut path_list:Vec<Value>= vec![];
    let folder_path = Path::new(path);
    if !folder_path.is_dir(){return Ok(Value::Null)}
    // 获取文件夹内所有项的迭代器
    let entries = fs::read_dir(folder_path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        // 获取项的名称
        let entry_name = entry.file_name();

        // 将名称转换为字符串
        let name = entry_name.to_str().unwrap();
        let path=format!("{}/{}",path,name);
        let folder_path = Path::new(&path);
        if folder_path.is_dir(){
            let json=json!({
                name:readfilenameloop(&path).unwrap()
            });
            path_list.push(json);
        }else{
            let name=name.to_string();
            path_list.push(serde_json::Value::String(name))
        }
    }
    let array_value:Value=serde_json::from_value(serde_json::Value::Array(path_list)).unwrap();
    let array_value=json!({
        path:array_value
    });
    Ok(array_value)
}
