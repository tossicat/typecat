//! # TypeCat lib
//!
//! 이 모듈은 기본적으로 TypeCat이 처리하는 함수를 정리하는 라이브러리입니다.
//!
//! ## 파일을 읽는 `read_flie()` 함수
//!
//! 이 함수는 `toml` 형식의 파일을 읽어오기 위한 함수입니다. 사용 방법은 다음과 같습니다.
//!
//! ```
//! use typecat::read_flie;
//!
//! let temp = read_flie("themes/test.toml".to_owned());
//! println!("{:?}", temp);
//! ```
//!

mod toml_parser;

use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_flie(file_name: String) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    reader
        .read_to_string(&mut data)
        .expect("Unable to read string");
    data
}

pub fn parsing_toml(contents: String) {
    let config: toml_parser::Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config.ip);
    println!("{:?}", config.port);
    println!("{:?}", config.keys.github);
    println!("{:?}", config.keys.travis.as_ref().unwrap());
}
