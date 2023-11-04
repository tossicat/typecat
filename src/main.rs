use typecat::markdown_parser;
use typecat::parse_toml;
use typecat::read_theme_file;
use typecat::test_table_parser;
use typecat::validate;

// CMD로 작동하기 위한 코드 시작
use clap::Parser as clap_parser;

#[derive(clap_parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file_names: Vec<String>,
}
// CMD로 작동하기 위한 코드 끝

fn read_files(file_names: Vec<String>) -> (String, String) {
    if file_names.len() == 1 {
        println!("파일 이름이 하나밖에 없습니다.");
        println!("md_file_name: {:?}", file_names[0]);
        let temp = read_theme_file(&file_names[1].to_owned());
        ("sss".to_string(), "aaa".to_string())
    } else {
        println!("파일 이름이 두 개!");
        println!("md_file_name: {:?}", file_names[0]);
        println!("toml_file_name: {:?}", file_names[1]);
        let temp = read_theme_file(&file_names[1].to_owned());
        println!("toml_file_content: {:?}", temp);
        match temp {
            Ok(e) => (e, "aaa".to_string()),
            Err(_) => todo!(),
        }
    }
}

fn main() {
    // CMD로 작동하기 위한 코드 시작
    // 사용법 아래와 같이 입력하면 됩니다.
    // cargo run -- test.md default.toml
    let cli = Cli::parse();
    println!("files_name: {:?}", cli.file_names);
    let temp = validate(&cli.file_names);
    match temp {
        Ok(e) => {
            read_files(e);
        }
        Err(e) => println!("{:?}", e),
    };

    // 마크다운 파서 테스트 시작
    markdown_parser();
    // 마크다운 파서 테스트 끝
}
