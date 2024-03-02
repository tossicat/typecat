//! # TypeCat lib
//!
//! 이 모듈은 기본적으로 TypeCat이 처리하는 함수를 정리하는 라이브러리입니다.
//!

use std::io;
extern crate pest;
#[macro_use]
extern crate pest_derive;
mod file_manager;
mod markdown_parser;
mod models;
mod pdf_printer;
mod toml_parser;

// 아래 두 상수는 원칙적으로는 `toml`이나 `ini` 형식의 파일로 설정을 저장하고
// 이를 프로그램이 실행될 때 읽어서 처리해야 하지만
// 현재는 개발 초기이기 때문에 이렇게 임시로 rust 모듈을 사용해서
// 설정 내용을 rust 형식으로 저장하고 사용하고자 합니다.
//
// 'D_T_FILE_LOC'은 'DEFAULT_THEME_FILE_LOCATAION' 의 약자:
// 필요한 테마 파일을 지정하지 않은 경우 이 파일을 이용합니다.
//
pub const D_T_FILE_LOC: &str = "themes/default.toml";
pub const DEFAULT_THEME_FOLDER: &str = "themes";

pub fn validate(file_names: &[String]) -> Result<(String, String), String> {
    file_manager::is_2_files_extensions_md_or_toml(file_names)
}

pub fn validate_toml_file(file_names: &String) -> Result<(bool, bool), io::Error> {
    file_manager::is_toml_file(file_names)
}

pub fn parse_toml(contents: String) {
    toml_parser::parsing_toml(contents);
}

pub fn markdown_parser() -> Vec<(markdown_parser::Rule, Vec<models::FragmentType>)> {
    markdown_parser::parse_markdown()
}

pub fn pdf_compiler(parsed_data: Vec<(markdown_parser::Rule, Vec<models::FragmentType>)>) {
    pdf_printer::compile(parsed_data);
}
