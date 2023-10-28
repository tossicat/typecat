//! # TypeCat lib
//!
//! 이 모듈은 기본적으로 TypeCat이 처리하는 함수를 정리하는 라이브러리입니다.
//!
extern crate pest;
#[macro_use]
extern crate pest_derive;
mod markdown_parser;
mod file_manager;
mod themes;
mod toml_parser;
mod test_table_parser;
mod models;

pub fn validate(file_names: &[String]) -> Result<Vec<String>, String> {
    file_manager::validate(file_names.to_vec())
}

pub fn identify_extension(file_name: &String, extension: &String) -> Result<bool, String> {
    file_manager::identify_extension(file_name, extension)
}

pub fn read_theme_file(file_name: &String) -> Result<String, String> {
    themes::read_toml_file(file_name)
}

pub fn parse_toml(contents: String) {
    toml_parser::parsing_toml(contents);
}

pub fn markdown_parser() {
    markdown_parser::parse_markdown();
}

pub fn test_table_parser() {
    test_table_parser::parse_table();
}