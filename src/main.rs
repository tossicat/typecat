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

fn main() {
    // CMD로 작동하기 위한 코드 시작
    // 사용법 아래와 같이 입력하면 됩니다.
    // cargo run -- test.md default.toml
    let cli = Cli::parse();
    // println!("files_name: {:?}", cli.file_names);
    let temp = validate(&cli.file_names);
    match temp {
        Ok(e) => {
            println!("md_file_name: {:?}", e[0]);
            println!("toml_file_name: {:?}", e[1]);
        }
        Err(e) => println!("{:?}", e),
    };

    // 마크다운 파서 테스트 시작
    markdown_parser();
    // 마크다운 파서 테스트 끝
}