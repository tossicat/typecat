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
    // CMD로 작동하기 위한 코드 끝

    // toml 형식 파일 테스트 시작
    // let toml_file_name = "test.toml";
    let toml_file_name = "default.toml";
    let test = read_theme_file(&toml_file_name.to_owned());
    match test {
        Ok(s) => println!("{:?}", parse_toml(s)),
        Err(e) => println!("{:?}", e),
    };
    // toml 형식 파일 테스트 끝

    // table 파서 테스트 시작
    println!("");
    println!("#### table 파서 테스트 시작 ####");
    test_table_parser();
    println!("#### table 파서 테스트 끝 ####");
    println!("");
    // table 파서 테스트 끝

    // 마크다운 파서 테스트 시작
    markdown_parser();
    // 마크다운 파서 테스트 끝
}
