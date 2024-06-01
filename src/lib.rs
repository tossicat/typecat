//! # TypeCat lib
//!
//! 이 모듈은 기본적으로 TypeCat이 처리하는 함수를 정리하는 라이브러리입니다.
//!

use std::io;
extern crate pest;
#[macro_use]
extern crate pest_derive;
// mod file_manager;
mod markdown;
mod pdf;
mod toml_parser;

use crate::markdown::datatypes::FragmentType;
use crate::markdown::md_parser::Rule;

// 아래 두 상수는 원칙적으로는 `toml`이나 `ini` 형식의 파일로 설정을 저장하고
// 이를 프로그램이 실행될 때 읽어서 처리해야 하지만
// 현재는 개발 초기이기 때문에 이렇게 임시로 rust 모듈을 사용해서
// 설정 내용을 rust 형식으로 저장하고 사용하고자 합니다.
//
// 'D_T_FILE_LOC'은 'DEFAULT_THEME_FILE_LOCATAION' 의 약자:
// 필요한 테마 파일을 지정하지 않은 경우 이 파일을 이용합니다.
//
pub const D_T_FILE_LOC: &str = "assets/themes/default.toml";
pub const DEFAULT_THEME_FOLDER: &str = "assets/themes";
pub const DEFAULT_FONT_FOLDER: &str = "assets/fonts";

pub fn validate(file_names: &[String]) -> Result<(String, String), String> {
    file_manager::is_2_files_extensions_md_or_toml(file_names)
}

pub fn validate_toml_file(file_names: &String) -> Result<(bool, bool), io::Error> {
    file_manager::is_toml_file(file_names, DEFAULT_THEME_FOLDER)
}

pub fn parse_toml(contents: String) {
    toml_parser::parsing_toml(contents);
}

pub fn parse_markdown(path: &str) -> Vec<(Rule, Vec<FragmentType>)> {
    let result = markdown::md_parser::parse(path);
    return result;
}

pub fn convert_pdf(output_path: &str, parsed_data: Vec<(Rule, Vec<FragmentType>)>) {
    pdf::pdf_converter::convert(output_path, parsed_data);
}
