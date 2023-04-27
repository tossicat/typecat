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
    pub keys: Keys,
}

#[derive(Deserialize)]
pub struct Keys {
    pub github: String,
    pub travis: Option<String>,
}
