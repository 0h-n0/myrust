use std::io::{Read, Write};

pub struct Config {
    values: Vec<(String, String)>,
}

pub struct KeyValueConfigService {}

impl Config {
    pub fn new(values: Vec<(String, String)>) -> Config {
        Config { values: values }
    }
}

impl KeyValueConfigService {
    pub fn new() -> KeyValueConfigService {
        KeyValueConfigService {}
    }
}

pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}

pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}

pub trait ConfigReader {
    fn write(&self, config: Config, from: &mut impl Read) -> std::io::Result<Config>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
