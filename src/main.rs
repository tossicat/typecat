use std::fs::read_to_string;

use typecat::parsing_toml;
use typecat::read_toml_file;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct MarkdownParser;

fn main() {
    // toml 형식 파일 테스트 시작
    // let temp = read_flie(&"themes/test.tom".to_owned());
    // println!("{:?}", temp);
    let toml_file_name = "tes.toml";
    let test = read_toml_file(&toml_file_name.to_owned());
    match test {
        Ok(s) => println!("{:?}", parsing_toml(s)),
        Err(e) => println!("{:?}", e),
    };
    // toml 형식 파일 테스트 끝
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
}
