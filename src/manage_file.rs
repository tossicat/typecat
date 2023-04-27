//! # file manager
//!
//! 이 프로젝트는 총 2개의 파일을 읽어와서 작업을 합니다. 이 모듈은 파일을 읽어올 때
//!  문제가 발생할 수 있는 것들을 처리하기 위한 것입니다. 예를 들어 읽은 파일의
//!  확장자(extension)가 적합한 것인지, 해당 파일이 들어 있는 폴더가 실재로 존재하는 것
//! 인지 등을 다루고 있습니다.

use std::path::Path;

/// 파일 확장자가 필요한 확장지인지 확인하는 함수
///
/// 만약 입력된 파일 이름에 들어 있는 확장자가 현재 필요한 확장자가 아니라면,
///  `Err()`을 이용해 에러 메세지를 반환합니다.
///  만약 같다면 `Ok(true)`을 반환하게 됩니다.
pub fn identify_extension(file_name: &String, extension: &String) -> Result<bool, String> {
    let path_extension = Path::new(&file_name);
    match path_extension.extension() {
        None => Err("No extensions.".to_string()),
        Some(path_extension) => match path_extension.to_str() == Some(extension) {
            true => Ok(true),
            false => {
                Err("The file extension is different from the extension you entered.".to_string())
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension() {
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명에 `toml` 확장자가 들어 있기 때문에 `true`을 반환합니다.
        let temp_path = "themes/test.toml";
        assert_eq!(
            Result::Ok(true),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명의 확장자가 `toml`이 아니기 때문에 에러 메세지를 반환합니다.
        let temp_path = "themes/test.tom";
        assert_eq!(
            Result::Err(
                "The file extension is different from the extension you entered.".to_string()
            ),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 입력된 파일명에 확장자가 `toml`형식이 들어 왔는지 확인하는 테스트입니다.
        // 현재는 파일명의 확장자가 `toml`이 아니기 때문에 에러 메세지를 반환합니다.
        let temp_path = "themes/test.md";
        assert_eq!(
            Result::Err(
                "The file extension is different from the extension you entered.".to_string()
            ),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
        // 현재는 파일명에 확장자가 없기 때문에 `false`를 반환합니다.
        let temp_path = "themes/test";
        assert_eq!(
            Result::Err("No extensions.".to_string()),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
    }
}
