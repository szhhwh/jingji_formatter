use std::{fs::OpenOptions, io::Read};
use log::debug;
use regex::Regex;

#[tauri::command]
pub fn format() -> Result<(), String> {
    debug!("进入格式化过程");
    let file = OpenOptions::new().read(true).append(true).open("C:\\Users\\szhh\\Desktop\\jingji_formatter\\test.txt");

    let regex = Regex::new(r"\r\n\r\n").unwrap(); // 创建正则表达式对象，用于匹配两个换行符

    match file {
        Ok(mut f) => {
            debug!("文件打开成功");
            let mut file_instring = String::new();
            let _ = f.read_to_string(&mut file_instring);
            println!("原始正文：{}", file_instring);
            let replaced_text = regex.replace_all(&file_instring, "\n•");
            println!("替换后正文：{}", replaced_text)
        }
        Err(e) => {
            debug!("文件打开失败: {}", e);
            return Err(e.to_string());
        }
    }
    Ok(())
}
