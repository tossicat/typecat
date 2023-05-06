use std::fs::read_to_string;

use typecat::parsing_toml;
use typecat::read_theme_file;
use typecat::validate;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct MarkdownParser;

// CMD로 작동하기 위한 코드 시작
use clap::Parser as clap_parser;

#[derive(clap_parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file_names: Vec<String>,
}
// CMD로 작동하기 위한 코드 끝

fn main() {
    let unparsed_file = read_to_string("test/test.md").expect("cannot read file");
    let file = MarkdownParser::parse(Rule::FILE, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for line in file.into_inner() {
        for sentence in line.into_inner() {
            println!("{:?}", sentence);
        }
    }
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
    let toml_file_name = "test.toml";
    let test = read_theme_file(&toml_file_name.to_owned());
    match test {
        Ok(s) => println!("{:?}", parsing_toml(s)),
        Err(e) => println!("{:?}", e),
    };
    // toml 형식 파일 테스트 끝
}
