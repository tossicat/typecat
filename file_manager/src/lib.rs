//! # file manager
//!
//! 이 프로젝트는 총 2개의 파일을 읽어와서 작업을 합니다. 이 모듈은 파일을 읽어올 때
//!  문제가 발생할 수 있는 것들을 처리하기 위한 것입니다. 예를 들어 읽은 파일의
//!  확장자(extension)가 적합한 것인지, 해당 파일이 들어 있는 폴더가 실재로 존재하는 것
//! 인지 등을 다루고 있습니다.
//!
//! 이 file manager 작업 공간만 테스트를 하려면 `cargo test`을 하시면 됩니다.

use std::path::PathBuf;
use std::{io, path::Path};

/// 특정 폴더에 들어 있는 폰트 파일 목록을 읽어 오는 함수
///
/// 특정 폴더에 들어 있는 폰트 파일들의 목록을 반환합니다.
/// 만약 해당 폴더안에 폰트 파일이 없으면 에러 메세지를 반환합니다.
pub fn read_assets_fonts_dir(fonts_dir: &String) -> Result<Vec<PathBuf>, String> {
    let mut file_name_list: Vec<PathBuf> = Vec::new();
    let path = Path::new(fonts_dir);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            file_name_list.push(entry.path());
        }
    }
    if file_name_list.len() == 0 {
        let temp_err_msg = format!("No! font file in {}", fonts_dir);
        return Err(temp_err_msg.to_string());
    } else {
        return Ok(file_name_list);
    }
}

#[derive(Debug, PartialEq)]
enum FileType {
    Toml,
    Md,
    None,
}

/// 입력된 `TOML` 파일이 있는지 없는지 검사하는 함수
///
/// 만약 입력된 파일 이름이 themes 폴더에
/// 있으면
/// 1 `Ok((true, false))`을 반환합니다.
/// 없으면
/// 2 `themes`라는 서브 폴더에 입력된 파일 이름이
/// 있으면
/// 2.1 `Ok((false, true))`을 반환합니다.
/// 없으면
/// 2.2 `Ok((false, false))`을 반환합니다.
/// 3 만약 현재 폴더와 `themes` 폴더 안에
/// 없으면 `Ok((false, false))`을 반환합니다.
/// 문제가 있으면 에러를 반환합니다.
pub fn is_toml_file(file_name: &String, path: &str) -> Result<(bool, bool), io::Error> {
    let is_temp_file_path = Path::new(&file_name).try_exists()?;
    match is_temp_file_path {
        true => Ok((true, false)),
        false => {
            let temp_new_path = String::from(path);
            let temp_new_path = temp_new_path + "/" + &file_name;
            let is_sub_temp_file_path = Path::new(&temp_new_path).try_exists()?;
            match is_sub_temp_file_path {
                true => Ok((false, true)),
                false => Ok((false, false)),
            }
        }
    }
}

/// 파일 1개의 확장자가 `md` 또는 `TOML`인지 확인하는 함수
///
/// 1. 만약 입력된 파일 이름에 들어 있는 확장자가 `md`이면
///   `Ok(FileType::Md)`을 반환합니다.
/// 2. 만약 입력된 파일 이름에 들어 있는 확장자가 `TOML`이면
///   `Ok(FileType::Toml)`을 반환합니다.
/// 3. 만약 입력된 파일 이름에 들어 있는 확장자가 `md`도 `TOML`도 아니라면,
///   `Err("Not all required file extensions.".to_string());`을 반환합니다.
/// 4. 만약 입력된 파일 이름에 들어 있는 확장자가 `md`이면서 `TOML`이라면,
///   이건 당연히 불가능합니다.
/// 5. 나머지 경우는 모두
///   `Err("No file extension.".to_string())`을 반환합니다.
///
/// 결국 확장자가 `md`나 `TOML`가 아니면 에러를 반환합니다.
fn is_file_extensions_md_or_toml(file_name: &str) -> Result<FileType, String> {
    let is_toml = identify_extension(file_name, &"toml".to_string());
    let is_md = identify_extension(file_name, &"md".to_string());
    if is_md == Some(true) {
        Ok(FileType::Md)
    } else if is_toml == Some(true) {
        return Ok(FileType::Toml);
    } else if is_md == Some(false) || is_toml == Some(false) {
        return Err("Not all required file extensions.".to_string());
    } else {
        return Err("No file extension.".to_string());
    }
}

/// 파일 2개의 확장자들이 `md` 또는 `TOML`인지 파악해
///
/// 만약 입력된 파일 이름에 들어 있는 확장자가 `md` 또는 `TOML`가 아니라면,
/// `Err()`을 이용해 에러 메세지를 반환합니다.
/// 이미 `is_file_extensions_md_or_toml()`함수가 입력된 파일의 확장자가
/// 이 두 확장자가 아니면 에러를 반환합니다. 따라서 이 함수에서는
/// `Ok(FileType::None)`을 반환하지 않습니다. 결론적으로 `temp_return`에는
/// `Ok(FileType::None)`이 들어갈 수 없습니다.
///
/// 참고로 아래 다음 조합은 모두 md 파일이 없는 조합이기 때문에
/// 모두 "md format file is required."이라는 에러를 반환합니다.
///
/// `[FileType::Toml, FileType::Toml]`
/// `[FileType::Toml, FileType::None]`
/// `[FileType::None, FileType::Toml]`
/// `[FileType::None, FileType::None]`
///
/// 따라서 모두 통합해서 처리합니다.

pub fn is_2_files_extensions_md_or_toml(file_names: &[String]) -> Result<(String, String), String> {
    let mut file_type_list = [FileType::None, FileType::None];
    let mut temp_return: (String, String) = ("".to_string(), "".to_string());
    for (i, file_name) in file_names.iter().enumerate() {
        // println!("file_name:{:?}", file_name);
        let temp_result = is_file_extensions_md_or_toml(file_name);
        match temp_result {
            Ok(FileType::Md) => {
                file_type_list[i] = FileType::Md;
                temp_return.0 = file_name.to_string();
            }
            Ok(FileType::Toml) => {
                file_type_list[i] = FileType::Toml;
                temp_return.1 = file_name.to_string();
            }
            Ok(FileType::None) => {
                file_type_list[i] = FileType::None;
            }
            Err(e) => return Err(e),
        };
    }
    match file_type_list {
        [FileType::Toml, FileType::Md] => Ok(temp_return),
        [FileType::Md, FileType::Toml] => Ok(temp_return),
        [FileType::Md, FileType::None] => Ok(temp_return),
        [FileType::None, FileType::Md] => Ok(temp_return),
        [FileType::Md, FileType::Md] => Err("Only one md format file is required.".to_string()),
        _ => Err("md format file is required.".to_string()),
    }
}

/// 파일 확장자가 필요한 확장지인지 확인하는 함수
///
/// 1. 만약 입력된 파일 이름에 들어 있는 확장자와
/// 1-1. 확인할 확장자와 같다면, `Some(true)`을 반환하게 됩니다.
/// 1-2. 확인할 확장자와 같지 않다면, `Some(false)`을 반환하게 됩니다.
/// 2. 만약 입력된 파일 이름에 들어 있는 확장자에 문제가 있으면,
/// 예를 들어 확장자 없다 와 같이
/// 이때는 `None`을 반환하게 됩니다.
/// 참고로 `to_lowercase()`을 이용하는 이유는 확장자 대문자, 소문자를 구분해서
/// 처리하는 것보다 단순하게 전체 문자열을 소문자로 바꿔서 처리하면 한 번에 해결할 수 있습니다.
pub fn identify_extension(file_name: &str, extension: &String) -> Option<bool> {
    let file_name = file_name.to_lowercase();
    let path_extension = Path::new(&file_name);
    match path_extension.extension() {
        None => None,
        Some(path_extension) => match path_extension.to_str().unwrap() == extension {
            true => Some(true),
            false => Some(false),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _identify_extension() {
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 첫번째 파일명에는 `ToML`와 같이 이상하게 확장자가 있습니다.
        // 그러나 이것도 소문자로 바꾸면 `toml`이기 때문에 `true`을 반환합니다.
        // 두번째 파일명에는 `TOML`와 같이 대문자로 들어 있기 때문에 무리없이 `true`을 반환
        let temp_path = "themes/test.ToML";
        assert_eq!(
            Some(true),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        let temp_path = "themes/test.TOML";
        assert_eq!(
            Some(true),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명에 `toml` 확장자가 들어 있기 때문에 `true`을 반환합니다.
        let temp_path = "themes/test.toml";
        assert_eq!(
            Some(true),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명의 확장자가 `toml`이 아니기 때문에 에러 메세지를 반환합니다.
        let temp_path = "themes/test.tom";
        assert_eq!(
            Some(false),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명의 확장자가 `toml`이 아니기 때문에 에러 메세지를 반환합니다.
        let temp_path = "themes/test.md";
        assert_eq!(
            Some(false),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 현재는 파일명에 확장자가 없기 때문에 `false`를 반환합니다.
        let temp_path = "themes/test";
        assert_eq!(
            None,
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 입력된 파일명에 확장자가 `md`형식이 들어 왔는지 확인하는 테스트입니다.
        // 첫번째 파일명에는 `Md`와 같이 이상하게 확장자가 있습니다.
        // 그러나 이것도 소문자로 바꾸면 `md`이기 때문에 `true`을 반환합니다.
        // 두번째 파일명에는 `MD`와 같이 대문자로 들어 있기 때문에 무리없이 `true`을 반환
        let temp_path = "themes/test.Md";
        assert_eq!(
            Some(true),
            identify_extension(&temp_path.to_string(), &"md".to_string())
        );
        let temp_path = "themes/test.MD";
        assert_eq!(
            Some(true),
            identify_extension(&temp_path.to_string(), &"md".to_string())
        );
        // 입력된 파일명에 확장자가 `md`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명에 `toml` 확장자가 들어 있기 때문에 `true`을 반환합니다.
        let temp_path = "themes/test.md";
        assert_eq!(
            Some(true),
            identify_extension(&temp_path.to_string(), &"md".to_string())
        );
        // 입력된 파일명에 확장자가 `md`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명의 확장자가 `toml`이 아니기 때문에 에러 메세지를 반환합니다.
        let temp_path = "themes/test.m";
        assert_eq!(
            Some(false),
            identify_extension(&temp_path.to_string(), &"md".to_string())
        );
        // 입력된 파일명에 확장자가 `md`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명의 확장자가 `md`이 아니기 때문에 에러 메세지를 반환합니다.
        let temp_path = "themes/test.toml";
        assert_eq!(
            Some(false),
            identify_extension(&temp_path.to_string(), &"md".to_string())
        );
        // 현재는 파일명에 확장자가 없기 때문에 `false`를 반환합니다.
        let temp_path = "themes/test";
        assert_eq!(
            None,
            identify_extension(&temp_path.to_string(), &"md".to_string())
        );
    }

    #[test]
    fn _is_file_extensions_md_or_toml() {
        // 입력된 파일명에 확장자가 `md`이 들어 왔는지 확인하는 테스트입니다.
        // `Err`을 반환하는 경우가 2가지가 있지만, 한 가지는 나올 수 없는 경우이기 때문에
        // 테스트하지 않습니다. 테스트하는 함수 주석을 참고하세요.
        // 현재 파일명에 `md` 확장자가 들어 있기 때문에 `Ok(true)`을 반환해야 합니다.
        let temp_file_name = "test.md".to_string();
        assert_eq!(
            Ok(FileType::Md),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
        // 현재 파일명에 `TOML` 확장자가 들어 있기 때문에 `Ok(false)`을 반환해야 합니다.
        // 즉 이 결과의 의미는 여기서에 사용가능한 확장자이자만 파일 이름이 1개만 들어온다면
        // `md` 형식 파일이 들어와야 한다는 의미입니다. 그러나 `toml` 형식이 들어 왔기 때문에
        // `false`를 추가해서 반환해야 합니다.
        let temp_file_name = "test.toml".to_string();
        assert_eq!(
            Ok(FileType::Toml),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
        // 현재 파일명에 `txt` 확장자가 들어 있기 때문에 `Err`을 반환하고
        // 아래와 같은 적절한 에러 메세지도 반환해야 합니다.
        let temp_file_name = "test.txt".to_string();
        assert_eq!(
            Err("Not all required file extensions.".to_string()),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
        // 파일 이름도 앞의 테스트오 다르게 바꾸고 확장자도 `jpg`로 변경해서 테스트합니다.
        // 파일명에 `jpg` 확장자 들어 있기 때문에 당연히 적절한 에러 메세지를 추가해 `Err`을 반환해야 합니다.
        let temp_file_name = "temp.jpg".to_string();
        assert_eq!(
            Err("Not all required file extensions.".to_string()),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
    }
}
