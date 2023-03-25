use serde::Deserialize;

fn main() {
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

    let config: Config = toml::from_str(
        r#"
   ip = '127.0.0.1'

   [keys]
   github = 'xxxxxxxxxxxxxxxxx'
   travis = 'yyyyyyyyyyyyyyyyy'
"#,
    )
    .unwrap();

    println!("{:?}", config.ip);
    println!("{:?}", config.port);
    println!("{:?}", config.keys.github);
    println!("{:?}", config.keys.travis.as_ref().unwrap());
}
