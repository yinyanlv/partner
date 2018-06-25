use std::fs::File;
use std::io::prelude::*;
use dotenv;
use toml;
use meval;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: App,
    pub mysql: Mysql,
    pub redis: Redis,
    pub cookie: Cookie
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub home_url: String,
    pub allowed_origin: String,
    #[serde(deserialize_with = "meval::de::as_f64")]
    pub cache_max_age: f64
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub url: String,
    #[serde(deserialize_with = "meval::de::as_f64")]
    pub ttl: f64
}

#[derive(Debug, Deserialize)]
pub struct Cookie {
    pub key: String,
    #[serde(deserialize_with = "meval::de::as_f64")]
    pub max_age: f64
}

impl Config {

    pub fn get() -> Config {

        let env = dotenv::var("APP_ENV").expect("APP_ENV must be set in .env file");

        let config_file_path = format!("config/{}.toml", env);

        let mut file = match File::open(&*config_file_path) {
            Ok(data) => data,
            Err(err) => panic!("no such file: {}, exception: {}", config_file_path, err)
        };

        let mut temp = String::new();
        
        match file.read_to_string(&mut temp) {
            Ok(_) => (),
            Err(err) => panic!("read file error: {}", err)
        };

        let config: Config = toml::from_str(&temp).unwrap();

        config
    }
}
