use log::debug;
use regex::Regex;
use serde_json::Value;
use std::{fs::OpenOptions, io::{Read, Write}, path::{Path, PathBuf}};

#[tauri::command]
pub fn format(file_list: Value) -> Result<(), String> {
    let file_array = serde_json::to_value(file_list).unwrap();
    debug!("转换后的json对象: {file_array}");

    let regex = Regex::new(r"\r\n\r\n").unwrap(); // 创建正则表达式对象，用于匹配两个换行符
    match file_array.as_array() {
        // 将Value(数组)转换为Vec<Value>
        Some(filepaths) => {
            for i in filepaths {
                if let Some(file_path) = i.get("path") {
                    // 逐个文件读取路径
                    debug!("文件路径: {file_path}");
                    let mut file_path = PathBuf::from(file_path.as_str().unwrap()); // 转换文件路径类型为PathBuf
                    let file = OpenOptions::new().read(true).append(true).open(&file_path); // 打开文件
                    match file {
                        Ok(mut file) => {
                            debug!("文件打开成功");
                            let mut file_instring = String::new();
                            let _ = file.read_to_string(&mut file_instring);
                            println!("原始正文：{}", file_instring);
                            let replaced_text = regex.replace_all(&file_instring, "\n•");
                            println!("替换后正文：{}", replaced_text);

                            // 写入文件
                            let mut filename = String::from(file_path.file_stem().unwrap().to_str().unwrap()); // 获取文件名称
                            filename.push_str("_formatted"); // 向文件名后添加后缀
                            file_path.set_file_name(filename); // 更新文件绝对路径
                            file_path.set_extension("txt");
                            let new_file = OpenOptions::new().write(true).create(true).open(file_path); // 创建新文件
                            if let Ok(mut file) = new_file {
                                let _ = file.write(replaced_text.as_bytes());
                            }
                            println!("文件写入成功");
                        }
                        Err(e) => {
                            debug!("文件打开失败: {}", e);
                            return Err(e.to_string());
                        }
                    }
                } else {
                    return Err("无法从前台获取文件路径".to_string());
                }
            }
        }
        None => {
            return Err("无法从前台获取文件路径".to_string());
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_file_name(path: String) -> Result<String, String> {
    let file_path = Path::new(&path);
    match file_path.file_stem() {
        Some(file_stem) => {
            let file_stem = file_stem.to_str().unwrap();
            Ok(file_stem.to_string())
        }
        None => Err("无法获取文件名".to_string()),
    }
}