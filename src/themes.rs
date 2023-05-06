//! # TypeCat lib
//!
//! 이 모듈은 기본적으로 TypeCat이 처리하는 함수를 정리하는 라이브러리입니다.
//!
//! ## 파일을 읽는 `read_flie()` 함수
//!
//! 이 함수는 `toml` 형식의 파일을 읽어오기 위한 함수입니다. 사용 방법은 다음과 같습니다.
//!
//! ```
//! use typecat::read_theme_file;
//!
//! let temp = read_theme_file(&"themes/test.toml".to_owned());
//! println!("{:?}", temp);
//! ```
//!
//!

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::file_types;

fn read_flie(file_name: &String) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    reader
        .read_to_string(&mut data)
        .expect("Unable to read string");
    data
}

/// `toml` 형식의 파일을 다루는 함수
///
/// 프로젝트에서 테마 파일은 `toml` 형식으로 되어 있습니다. 이 프로젝트에서
///  테마 파일이란 문서를 스타일링을 정의해 놓은 역할을 합니다. 이 함수가 이 형식의 파일을
///  읽기 위한 것입니다. 이 함수는 이때 2가지를 검사해서 처리합니다.
///  
/// 첫 번째로 우선 확장자가 입력된 파일 이름의 확장자가 "toml"인지 확인합니다.
///  아니면 다음과 같은 메세지를 반환합니다.
///
/// `"The file extension is different from the extension you entered."`
///
/// 두 번재로 해당 파일이 있는지 없는지를 두 번 검사합니다.
///  이런 테마 파일들은 기본적으로 `/themes`라는 폴더에 들어 있습니다.
///  일반적으로 사용자는 출력하고자 하는 내용이 들어 있는 특정 `md` 형식 파일과
///  스타일을 담당하는 `toml` 형식 파일, 이 두 가지 파일을 지정할 것입니다.
///  이때 이 함수는 우선 사용자가 지정한 `toml` 형식 파일이 현재 폴더에 있는지 확인합니다.
///  만약 그 파일이 있으면 바로 그 파일을 읽어 처리합니다. 그런데 만약 없으면,
///  `/themes`라는 폴더에 그 파일이 있는지 확인해, 있으면 바로 그 파일을 읽어 처리합니다.
///  만약 없으면 다음 에러 메세지를 반환합니다.
///  
/// `"No such file"`
///
pub fn read_toml_file(file_name: &String) -> Result<String, String> {
    let temp_extension = "toml".to_string();
    let sub_path = "themes".to_string();
    let temp_file_path = Path::new(&file_name);
    // println!("{:?}",manage_file::identify_extension(file_name, &temp_extension));
    let temp = match file_types::identify_extension(file_name, &temp_extension) {
        Result::Ok(true) => match temp_file_path.try_exists() {
            Result::Ok(true) => Ok(read_flie(file_name)),
            Result::Ok(false) => {
                let temp_path = Path::new(&sub_path).join(file_name);
                // println!("{:?}", temp_path);
                let temp_path_str = String::from(temp_path.to_string_lossy());
                match temp_path.try_exists() {
                    Result::Ok(true) => Ok(read_flie(&temp_path_str)),
                    Result::Ok(false) => Err("No such file".to_string()),
                    Result::Err(e) => Err(e.to_string()),
                }
            }
            Result::Err(e) => Err(e.to_string()),
        },
        Result::Ok(false) => Err("No such file".to_string()),
        Result::Err(e) => Err(e),
    };
    temp
}
