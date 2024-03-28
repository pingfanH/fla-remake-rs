use std::{io::{self},process::exit};
use utlis::{PingDebug,unzip};
mod utlis;
use anyhow::Result;
use std::path::Path;
mod xml;
mod mode;
mod file;
use mode::*;
use colored::*;
use std::process::Command;
const ZIP:&[u8]=include_bytes!("../ZIP.exe");
fn main()-> Result<()> {
    zip_exe();
    let name=find()?;
    let mode=remake_mode()?;
    let line=lines()?;
    unzip_fla(&name);
    match mode.as_str(){
        "1"=>{mode_one(&name,&line)},
        "2"=>{mode_two(&name,&line)},
        _=>{"unknow mode".einfo();exit(0x0100)}
    }
    command(&line);
    Ok(())
}
fn unzip_fla(name:&str){
    let path=Path::new(name);
    if path.is_dir(){return;}
    
    let file_name=name.split(".").next().unwrap();
    let _ = unzip(&name, &file_name);
}
fn find()->Result<String>{
    println!("{} {} :","input fla name","(no .fla)");
    let mut name=String::new();
    match io::stdin().read_line(&mut name){
        Ok(_)=>{},
        Err(err)=>{err.einfo();exit(0x0100)}
    };
    let name=name[0..name.len()-2].to_string();
    let file_path_str:String;
    if Path::new(&name).exists() {
        file_path_str=name;
    } else {
        let name = format!("{}.fla", name);
        file_path_str = name;
    };
    let file_path=Path::new(&file_path_str);
    if file_path.exists(){

        "find!".info();
        return Ok(file_path_str);
    }else{
        "fla not find!".einfo();
        exit(0x0100)
    }
}
fn remake_mode()->Result<String>{
    println!("{}","choose remake mode");
    println!("{}","1:Specify exact name");
    println!("{}","2:Extract export names containing input characters");
    println!("{}","Use space splitting");
    // println!("1:指定确切名称");
    // println!("2:提取所有包含输入字符的导出名称");
    // println!("均可使用空格分割");
    let mut mode=String::new();
    match io::stdin().read_line(&mut mode){
        Ok(_)=>{},
        Err(err)=>{err.einfo();exit(0x0100)}
    };
    mode=mode[0..mode.len()-2].to_string();
    Ok(mode)
}
fn lines()->Result<String>{
    println!("{}","input args");
    // println!("input args");
    let mut line=String::new();
    match io::stdin().read_line(&mut line){
        Ok(_)=>{},
        Err(err)=>{err.einfo();exit(0x0100)}
    };
    line=line[0..line.len()-2].to_string();
    Ok(line)
}
fn zip_exe(){
    let zip_path="./ZIP.exe";
    let path=Path::new(zip_path);
    if path.is_file(){return;}
    let _ = utlis::write("ZIP.exe", ZIP);
}
fn command(args:&str){
    let _ = Command::new("./ZIP.exe")
        .arg(args)
        .output();
}
#[test]
fn test(){

}