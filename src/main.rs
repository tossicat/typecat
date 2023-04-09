use std::fs::{File, read_to_string};
use std::path::PathBuf;
use std::io::{Error, Read, BufReader};

use serde::Deserialize;

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

fn read_flie(file_name: String) -> Result<(), Error> {
    let file_name = PathBuf::from(file_name);
    println!("Is {:?} exist?: {:?}", file_name, file_name.exists());
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    // print!("{}", contents);
    let config: Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config.ip);
    println!("{:?}", config.port);
    println!("{:?}", config.keys.github);
    println!("{:?}", config.keys.travis.as_ref().unwrap());
    Ok(())
}

fn main() {
    read_flie("themes/test.toml".to_owned());
    let unparsed_file = read_to_string("test/test.md").expect("cannot read file");
    let file = MarkdownParser::parse(Rule::FILE, &unparsed_file).expect("unsuccessful parse").next().unwrap();
    for line in file.into_inner() {
        println!("{:?}", line);
    }
}