//! # TOML parser for document style
//!
//! 이 모듈은 `TOML` 형식으로 되어 있는 스타일 설정 파일을 읽어 그 속에 들어 있는 내용을
//!  규정하는 모듈 입니다.
//! 현재는 파일을 읽어오는 코드에 집중하기 때문에 아래 코드는 더미 코드에 가깝습니다. 참고하세요.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
struct Config {
    global_integer: Option<u64>,
    h1: H1,
    // page: Page,
}

/// Sub-structs are decoded from tables, so this will decode from the `[server]`
/// table.
///
/// Again, each field is optional, meaning they don't have to be present.
#[derive(Debug, Deserialize)]
struct H1 {
    font: Option<String>,
    font_size: u64,
    left_margin: Option<u64>,
    right_margin: Option<u64>,
    top_margin: Option<u64>,
    bottom_margin: Option<u64>,
    indent: Option<u64>,
}

// #[derive(Debug, Deserialize)]
// struct Page {
//     font: String,
//     font_size: u64,
//     left_margin: u64,
//     right_margin: u64,
//     top_margin: u64,
//     bottom_margin: u64,
//     indent: Option<u64>,
// }
pub fn parsing_toml(contents: String) {
    let config: Config = toml::from_str(&contents).unwrap();
    // let decoded: Config = toml::from_str(toml_str).unwrap();
    println!("{:?}", config.global_integer);
    // println!("{:?}", config.paragraph.font);
    println!("H1 폰트 크기: {:?}", config.h1.font_size);
    println!("H1 폰트 이름: {:?}", config.h1.font);
    println!("H1 아래쪽 마진: {:?}", config.h1.bottom_margin);
    // println!("Page 아래쪽 마진: {:?}", config.page.bottom_margin);
}
