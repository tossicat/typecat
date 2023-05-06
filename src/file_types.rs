//! # file manager
//!
//! 이 프로젝트는 총 2개의 파일을 읽어와서 작업을 합니다. 이 모듈은 파일을 읽어올 때
//!  문제가 발생할 수 있는 것들을 처리하기 위한 것입니다. 예를 들어 읽은 파일의
//!  확장자(extension)가 적합한 것인지, 해당 파일이 들어 있는 폴더가 실재로 존재하는 것
//! 인지 등을 다루고 있습니다.

use std::path::Path;

#[derive(Debug)]
enum FileType {
    Toml,
    Md,
    Others,
}

/// 파일 확장자가 필요한 확장지인지 확인하고 적절한 파일 이름 목록을 반환하는 함수
///
/// - 만약 입력된 파일 이름 목록에 들어 있는 확장자가 현재 필요한 확장자(`md`, `TOML`)가 아니라면,
///  `Err()`을 이용해 에러 메세지를 반환합니다.
/// - 만약 모두 필요한 확장자라면,
///     - 먄약 파일 이름이 1개면,
///         - 이 파일의 확장자가 `md`라면, 그 파일 이름 목록을 `Ok()`을 이용해 반환하게 됩니다.
///     - 먄약 파일 이름이 2개면,
///         - 둘 다 확장자가 같다면, `Err()`을 이용해 에러 메세지를 반환합니다.
///         - 한 개는 `md`, 나머지는 `TOML`이면, `md`, `TOML`순서로 된 목록을 `Ok()`을 이용해 반환
///     - 먄약 파일 이름이 2개 이상이면,
///         - `Err()`을 이용해 에러 메세지를 반환합니다.
pub fn validate(file_names: Vec<String>) -> Result<Vec<String>, String> {
    match file_names.len() {
        0 => Err("There are no file names.".to_string()),
        1 => {
            let temp = is_file_extensions_md_or_toml(&file_names[0]);
            match temp {
                Ok(true) => Ok(file_names),
                Ok(false) => Err("There is no md format file. only TOML format file!".to_string()),
                Err(e) => Err(e),
            }
        }
        2 => {
            let temp = is_2_files_extensions_md_or_toml(&file_names);
            match temp {
                Ok(_) => temp,
                Err(e) => Err(e),
            }
        }
        _ => Err("Too many file names entered.".to_string()),
    }
}

/// 파일 1개의 확장자가 `md` 또는 `TOML`인지 확인하는 함수
///
/// 1. 만약 입력된 파일 이름에 들어 있는 확장자가 `md`도 `TOML`도 아니라면,
///  `Err()`을 이용해 에러 메세지를 반환합니다.
/// 2. 만약 입력된 파일 이름에 들어 있는 확장자가 `md`이면서 `TOML`이라면,
///  `Err()`을 이용해 에러 메세지를 반환합니다. 이건 당연히 불가능합니다.
/// 3. 만약 입력된 파일 이름에 들어 있는 확장자가 `md`이면
///   `Ok(true)`을 반환합니다.
/// 4. 만약 입력된 파일 이름에 들어 있는 확장자가 `TOML`이면
///   `Ok(false)`을 반환합니다.
fn is_file_extensions_md_or_toml(file_name: &String) -> Result<bool, String> {
    // 파일 확장자가 `toml`이면 indicater.1 = true
    // 파일 확장자가 `md`이면 indicater.0 = true
    let mut indicater: (bool, bool) = (false, false);
    let is_toml = identify_extension(file_name, &"toml".to_string());
    let is_md = identify_extension(file_name, &"md".to_string());
    if is_md == Ok(true) {
        indicater.0 = true;
    } else if is_md == Err("No file extensions.".to_string()) {
        return is_md;
    } else {
        indicater.0 = false;
    }
    if is_toml == Ok(true) {
        indicater.1 = true;
    } else if is_toml == Err("No file extensions.".to_string()) {
        return is_toml;
    } else {
        indicater.1 = false;
    }
    // println!("is_md_or_TOML: {:?}", indicater);
    match indicater {
        (true, true) => Err("It is a md format file and a TOML format file!".to_string()),
        (true, false) => Ok(true),
        (false, true) => Ok(false),
        (false, false) => Err("Not even a md format file as a TOML format file!".to_string()),
    }
}

/// 파일 2개의 확장자들이 `md` 또는 `TOML`인지 확인하는 함수
///
/// 만약 입력된 파일 이름에 들어 있는 확장자가 `md` 또는 `TOML`가 아니라면,
///  `Err()`을 이용해 에러 메세지를 반환합니다.
///  만약 같다면 `Ok(true)`을 반환하게 됩니다.
fn is_2_files_extensions_md_or_toml(file_names: &[String]) -> Result<Vec<String>, String> {
    let mut file_type_list = [FileType::Others, FileType::Others];
    let mut temp_result: Vec<String> = Vec::new();
    for (i, file_name) in file_names.iter().enumerate() {
        let temp_result = is_file_extensions_md_or_toml(file_name);
        match temp_result {
            Ok(true) => {
                file_type_list[i] = FileType::Md;
            }
            Ok(false) => {
                file_type_list[i] = FileType::Toml;
            }
            Err(e) => println!("Err: {:?}", e),
        };
    }
    match file_type_list {
        [FileType::Toml, FileType::Toml] => Err("All are TOML format files.".to_string()),
        [FileType::Toml, FileType::Md] => {
            temp_result.push(file_names[1].clone());
            temp_result.push(file_names[0].clone());
            Ok(temp_result)
        }
        [FileType::Md, FileType::Toml] => {
            temp_result.push(file_names[0].clone());
            temp_result.push(file_names[1].clone());
            Ok(temp_result)
        }
        [FileType::Md, FileType::Md] => Err("All are MD format files.".to_string()),
        _ => Err("Not even a md format file as a TOML format file!".to_string()),
    }
}

/// 파일 확장자가 필요한 확장지인지 확인하는 함수
///
/// 만약 입력된 파일 이름에 들어 있는 확장자가 현재 필요한 확장자가 아니라면,
///  `Err()`을 이용해 에러 메세지를 반환합니다.
///  만약 같다면 `Ok(true)`을 반환하게 됩니다.
pub fn identify_extension(file_name: &String, extension: &String) -> Result<bool, String> {
    let path_extension = Path::new(&file_name);
    match path_extension.extension() {
        None => Err("No file extensions.".to_string()),
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
    fn _identify_extension() {
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
            Result::Err("No file extensions.".to_string()),
            identify_extension(&temp_path.to_string(), &"toml".to_string())
        );
    }

    #[test]
    fn _is_file_extensions_md_or_toml() {
        // 입력된 파일명에 확장자가 `mt`이 들어 왔는지 확인하는 테스트입니다.
        // `Err`을 반환하는 경우가 2가지가 있지만, 한 가지는 나올 수 없는 경우이기 때문에
        // 테스트하지 않습니다. 테스트하는 함수 주석을 참고하세요.
        // 현재 파일명에 `md` 확장자가 들어 있기 때문에 `Ok(true)`을 반환해야 합니다.
        let temp_file_name = "test.md".to_string();
        assert_eq!(
            Result::Ok(true),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
        // 현재 파일명에 `TOML` 확장자가 들어 있기 때문에 `Ok(false)`을 반환해야 합니다.
        // 즉 이 결과의 의미는 여기서에 사용가능한 확장자이자만 파일 이름이 1개만 들어온다면
        // `md` 형식 파일이 들어와야 한다는 의미입니다. 그러나 `toml` 형식이 들어 왔기 때문에
        // `false`를 추가해서 반환해야 합니다.
        let temp_file_name = "test.toml".to_string();
        assert_eq!(
            Result::Ok(false),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
        // 현재 파일명에 `txt` 확장자가 들어 있기 때문에 `Err`을 반환하고
        // 아래와 같은 적절한 에러 메세지도 반환해야 합니다.
        let temp_file_name = "test.txt".to_string();
        assert_eq!(
            Err("Not even a md format file as a TOML format file!".to_string()),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
        // 파일 이름도 앞의 테스트오 다르게 바꾸고 확장자도 `jpg`로 변경해서 테스트합니다.
        // 파일명에 `jpg` 확장자 들어 있기 때문에 당연히 적절한 에러 메세지를 추가해 `Err`을 반환해야 합니다.
        let temp_file_name = "temp.jpg".to_string();
        assert_eq!(
            Err("Not even a md format file as a TOML format file!".to_string()),
            is_file_extensions_md_or_toml(&temp_file_name)
        );
    }
}
