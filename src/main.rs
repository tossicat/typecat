use std::fs::File;
use std::io::{Error, Read, BufReader};

use serde::Deserialize;

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
    let file_name = file_name;
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
}
