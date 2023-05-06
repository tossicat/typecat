//! # TypeCat lib
//!
//! 이 모듈은 기본적으로 TypeCat이 처리하는 함수를 정리하는 라이브러리입니다.
//!

mod manage_file;
mod themes;
mod toml_parser;

pub fn validate(file_names: &[String]) -> Result<Vec<String>, String> {
    manage_file::validate(file_names.to_vec())
}

pub fn identify_extension(file_name: &String, extension: &String) -> Result<bool, String> {
    manage_file::identify_extension(file_name, extension)
}

pub fn read_theme_file(file_name: &String) -> Result<String, String> {
    themes::read_toml_file(file_name)
}
pub fn parsing_toml(contents: String) {
    let config: toml_parser::Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config.ip);
    println!("{:?}", config.port);
    println!("{:?}", config.keys.github);
    println!("{:?}", config.keys.travis.as_ref().unwrap());
}
