//! # TOML parser for document style
//!
//! 이 모듈은 `TOML` 형식으로 되어 있는 스타일 설정 파일을 읽어 그 속에 들어 있는 내용을
//!  규정하는 모듈 입니다.
//! 현재는 파일을 읽어오는 코드에 집중하기 때문에 아래 코드는 더미 코드에 가깝습니다. 참고하세요.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: Option<u16>,
    pub page: Page,
    pub paragraph: Paragraph,
    pub h1:H1,
}

#[derive(Deserialize)]
pub struct Page {
    pub margin : i32,
    pub font: String,
    // pub travis: Option<String>,
}

#[derive(Deserialize)]
pub struct Paragraph {
    pub margin : i32,
    pub font: String,
    // pub travis: Option<String>,
}

#[derive(Deserialize)]
pub struct H1 {
    pub margin : i32,
    pub font: String,
    // pub travis: Option<String>,
}

pub fn parsing_toml(contents: String) {
    let config: Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config.ip);
    println!("{:?}", config.port);
    // println!("{:?}", config.keys.travis.as_ref().unwrap());
    println!("{:?}", config.page.font);
    println!("{:?}", config.page.margin);
    println!("{:?}", config.paragraph.margin);
    println!("{:?}", config.paragraph.font);
    println!("{:?}", config.h1.margin);
    println!("{:?}", config.h1.font);
}