use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use chrono::Local;
use jingji_formatter::error::AppError;
use log::{debug, error};

pub struct Filefmt {
    pub path: PathBuf, // 文件路径
    text: String,
}

impl Filefmt {
    /// 传入需要格式化的文件路径
    ///
    /// # Input parameters 入参
    /// - path: PathBuf
    ///
    /// # Return 返回值
    /// - Self
    pub fn new(path: PathBuf) -> Self {
        debug!("New Filefmt");
        Self {
            path: PathBuf::from(&path),
            text: String::new(),
        }
    }

    /// 尝试打开路径对应的文件
    ///
    /// # Input parameters 入参
    /// - None
    ///
    /// # Return 返回值
    /// - Result<(), AppError>
    pub fn openfile(&mut self) -> Result<(), AppError> {
        let tryfile = OpenOptions::new().read(true).append(true).open(&self.path);
        match tryfile {
            Ok(mut v) => {
                let mut file_content = String::new();
                let _ = v.read_to_string(&mut file_content);
                self.text = file_content; // 存储文件正文到Struct中
            }
            Err(e) => return Err(AppError::Err(e.to_string())),
        }
        Ok(())
    }

    /// 写入已经格式化的文件
    ///
    /// # Input parameters 入参
    /// - 新文件后缀
    ///
    /// # Return 返回值
    /// - Result<(), AppError>
    pub fn writetofile(&self, ext: &str) -> Result<(), AppError> {
        let timestamp = Local::now().timestamp().to_string();
        let mut file_name = String::from(self.path.file_stem().unwrap().to_str().unwrap());
        let mut file_path = PathBuf::from(&self.path);
        let file_ext = String::from(file_path.extension().unwrap().to_string_lossy());
        file_name.push_str(ext);
        file_name.push_str(&timestamp);
        file_path.set_file_name(file_name);
        file_path.set_extension(file_ext);
        let new_file = OpenOptions::new().write(true).create(true).open(file_path);
        match new_file {
            Ok(mut file) => {
                let _ = file.write_all(self.text.as_bytes());
                return Ok(());
            }
            Err(e) => return Err(AppError::Err(e.to_string())),
        }
    }

    /// 格式化文本
    ///
    /// # Input parameters 入参
    /// - reg<&str>：正则表达式
    /// - rep<T: Replacer>：替换为
    fn fmt<T: regex::Replacer>(&mut self, reg: &str, rep: T) -> Result<(), AppError> {
        let regex = regex::Regex::new(reg).unwrap();
        let text = regex.replace_all(&self.text, rep);
        self.text = text.to_string();
        Ok(())
    }

    pub fn fmt_bullets(&mut self) -> Result<(), AppError> {
        let reg = r"(?:\r?\n){2,}";
        let rep = "\n•";
        self.fmt(reg, rep)?;
        Ok(())
    }

    pub fn fmt_cleanspace(&mut self) -> Result<(), AppError> {
        let reg = r"(?m)^[ 　]+";
        let rep = "";
        self.fmt(reg, rep)?;
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum Fmttype {
    Default,
    Bullets,
    Cleanspace,
}

pub fn formating(path: PathBuf, tp: &Fmttype) -> Result<(), AppError> {
    let mut file = Filefmt::new(path);
    match file.openfile() {
        Ok(_) => {
            if &Fmttype::Bullets == tp {
                file.fmt_bullets()?;
                file.writetofile("_bullets_")
            } else if &Fmttype::Cleanspace == tp {
                file.fmt_cleanspace()?;
                file.writetofile("_cleanspace_")
            } else {
                error!("错误的格式化类型");
                return Err(AppError::Err("错误的格式化类型".to_string()));
            }
            
        }
        Err(e) => {
            debug!("文件打开失败: {}", e);
            return Err(AppError::Err(e.to_string()));
        }
    }
}
