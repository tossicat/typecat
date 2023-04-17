use std::fs::read_to_string;

use serde::Deserialize;

use typecat::read_flie;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "markdown.pest"]
pub struct MarkdownParser;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn parsing_toml(contents: String) {
    let config: Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config.ip);
    println!("{:?}", config.port);
    println!("{:?}", config.keys.github);
    println!("{:?}", config.keys.travis.as_ref().unwrap());
}

fn main() {
    // toml 형식 파일 테스트 시작
    let temp = read_flie("themes/test.toml".to_owned());
    println!("{:?}", parsing_toml(temp));
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
