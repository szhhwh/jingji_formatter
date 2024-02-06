use crate::app::formatter::{formating, Fmttype};
use log::{debug, error};
use serde_json::Value;
use std::path::{Path, PathBuf};

#[tauri::command]
pub fn format(file_list: Value, tp: Value) -> Result<(), String> {
    // 将入参转换为Value
    let file_array = serde_json::to_value(file_list).unwrap();
    let fmttype_json = serde_json::to_value(tp).unwrap();
    let fmttype_str = fmttype_json.as_str().unwrap();
    debug!("转换后的json对象1: {file_array}");
    debug!("转换后的json对象2: {fmttype_str}");

    let fmttype: Fmttype;
    if "Bullets" == fmttype_str {
        fmttype = Fmttype::Bullets;
    } else if "Cleanspace" == fmttype_str {
        fmttype = Fmttype::Cleanspace;
    } else {
        fmttype = Fmttype::Default;
    }

    match file_array.as_array() {
        // 将Value(数组)转换为Vec<Value>
        Some(filepaths) => {
            for i in filepaths {
                if let Some(file_path) = i.get("path") {
                    // 逐个文件读取路径
                    debug!("文件路径: {file_path}");
                    let file_path = PathBuf::from(file_path.as_str().unwrap()); // 转换文件路径类型为PathBuf
                    match formating(file_path, &fmttype) {
                        Ok(_) => {
                            debug!("文件格式化成功");
                        }
                        Err(e) => {
                            error!("文件格式化失败：{e}");
                            return Err("文件格式化失败: {e}".to_string());
                        }
                    }
                } else {
                    error!("无法从前台获取文件路径");
                    return Err("无法从前台获取文件路径".to_string());
                }
            }
        }
        None => {
            error!("无法从前台获取文件路径");
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
