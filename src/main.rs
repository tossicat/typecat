use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::path::PathBuf;

use typecat::{convert_pdf, parse_markdown, parse_toml, validate};
use typecat::{validate_toml_file, DEFAULT_FONT_FOLDER, D_T_FILE_LOC};

// CMD로 작동하기 위한 코드 시작
use clap::Parser as clap_parser;

#[derive(clap_parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file_names: Vec<String>,
}
// CMD로 작동하기 위한 코드 끝
///
/// 입력된 파일을 읽어 그 내용을 반환하는 함수
///
fn read_flie(file_name: &String) -> String {
    let mut data = String::new();
    let f = File::open(file_name).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    reader
        .read_to_string(&mut data)
        .expect("Unable to read string");
    data
}

/// 디폴트 toml 파일을 읽어 오는 함수
///
/// 이 함수를 따로 만든 이유는 디폴트 toml 파일을 읽어 온다는
/// 것을 표시하기 위한 `println!()`이 필요하기 때문입니다.
fn read_default_toml_file() -> String {
    let temp_path = D_T_FILE_LOC;
    println!("reading `{}`", temp_path);
    read_flie(&temp_path.to_string())
}

/// 디폴트 font 돌더에서 폰트 파일 목록을 읽어 오는 함수
///
/// 이 프로젝트에서 사용하는 폰트가 들어 있는 폴더 위치는
/// `DEFAULT_THEME_FOLDER`에 들어 있습니다.
/// 이곳에 들어 있는 폰트 파일을 목록으로 반환합니다.
/// 만약 해당 폴더안에 폰트 파일이 없으면 에러 메세지를 반환합니다.
fn read_assets_fonts_dir() -> Result<Vec<PathBuf>, String> {
    let mut file_name_list: Vec<PathBuf> = Vec::new();
    let path = Path::new(DEFAULT_FONT_FOLDER);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            file_name_list.push(entry.path());
        }
    }
    if file_name_list.len() == 0 {
        let temp_err_msg = format!("No! font file in {}", DEFAULT_FONT_FOLDER);
        return Err(temp_err_msg.to_string());
    } else {
        return Ok(file_name_list);
    }
}

/// CMD로 입력된 파일을 읽어 오는 함수
///
/// 이 함수는 CMD로 작동하기 위해 필요한 함수입니다.
/// CMD를 작동하기 위해서는 현재 개발 시점에서는 다음과 같은 명령어로 작동합니다.
///
/// `cargo run test.md test.toml`
///
/// 이때 통상적으로 `md` 형식의 파일 하나와 `TOML` 형식의 파일 하나를 입력하게 됩니다.
/// `md` 형식의 파일은 문서 컨탠츠가 들어 있는 파입니다.
/// `TOML` 형식의 문서 스타일을 지정하는 내용이 들어 있는 파일입니다.
/// 현재 개발을 위해 다음과 같이 두 폴더에 테스트 파일 2개가 들어 있습니다.
///
/// `cargo run test/pdf.md themes/test.toml`
///
/// 이렇게 작동하면 테스트를 할 수 있습니다. 실제로 작동할 때는 이 두 파일들을 사용하지 않습니다.
/// 실제로 사용자가 만들 파일을 사용할 수 있게 됩니다. 만약 `TOML` 형식의 파일 이름을 다음과 같이
///
/// `cargo run test/pdf.md`
///
/// 실행한다면 미리 만들어서 저장되어 있는 `themes/default.toml`을 읽어와 사용하게 됩니다.
/// 참고로 `TOML` 형식의 파일은 폴더를 입력하지 않고 파일 이름만 입력해도
/// 자동적으로 `themes` 폴더 안에도 있는지 확인한 후, 있으면 이 폴더 안에 들어 있는 파일을 열고
/// 이 폴더에도 없으면 `themes/default.toml`을 읽어와 사용하게 됩니다.

fn read_files(file_names: Vec<String>) -> Result<(String, String), String> {
    let file_name_list = validate(&file_names);
    // println!("file_name_list:{:?}", file_name_list);
    let mut temp_return: (String, String) = ("".to_string(), "".to_string());
    match file_name_list {
        Ok(e) => {
            println!("md_file_name:{}", e.0);
            let temp_md = read_flie(&e.0);
            temp_return.0 = temp_md;
            if e.1.is_empty() {
                println!("TOML 파일 이름을 입력하지 않았습니다.");
                let temp_toml = read_default_toml_file();
                temp_return.1 = temp_toml;
            } else {
                println!("toml_file_name:{}", e.0);
                let is_toml_file = validate_toml_file(&e.1);
                match is_toml_file {
                    Ok(m) => match m {
                        (true, false) => {
                            let temp_toml = read_flie(&e.1);
                            temp_return.1 = temp_toml;
                        }
                        (false, true) => {
                            let temp_toml = read_default_toml_file();
                            temp_return.1 = temp_toml;
                        }
                        _ => println!("{} 파일이 있습니까?", e.1),
                    },
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err(e) => return Err(e),
    }
    Ok(temp_return)
}

fn main() {
    // CMD로 작동하기 위한 코드 시작
    // 사용법 아래와 같이 입력하면 됩니다.
    // cargo run test/pdf.md test.toml
    // let cli = Cli::parse();
    // println!("files_names: {:?}", cli.file_names);
    // let temp_string = read_files(cli.file_names);
    // match temp_string {
    //     Ok(m) => {
    //         println!("\n~~~ md 파일 내용 \n \n {} \n~~~", m.0);
    //         println!("\n~~~ toml 파일 내용 \n \n {} \n~~~", m.1);
    //     }
    //     Err(_) => todo!(),
    // }
    // 마크다운 파서 테스트 시작
    let md_path = "test/pdf.md";
    let output_path = "test.pdf";
    let parsed_data = parse_markdown(md_path);
    convert_pdf(output_path, parsed_data);
    // 마크다운 파서 테스트 끝
    // CMD로 작동하기 위한 코드 시작
    // 사용법 아래와 같이 입력하면 됩니다.
    // cargo run test/pdf.md default.toml
    let cli = Cli::parse();
    println!("files_names: {:?}", cli.file_names);
    let temp_string = read_files(cli.file_names);
    match temp_string {
        Ok(m) => {
            println!("\n~~~ md 파일 내용 \n \n {} \n~~~", m.0);
            println!("\n~~~ toml 파일 내용 \n \n {:?} \n~~~", m.1);
            parse_toml(m.1);
        }
        Err(_) => todo!(),
    }
    // 아래 2줄은 폰트 폴더 안에 들어 있는 폰트 파일을 읽어오는 것을 테스트 하기 위한
    // 코드입니다. 폰트 관련 작업이 어느 정도 진행되면 지우겠습니다.
    let temp_fn = read_assets_fonts_dir();
    println!("font files names: \n {:?}", temp_fn);
}
