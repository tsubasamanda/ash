pub struct Config {

}

impl Default for Config {
    fn default() -> Self {
        Self {  }
    }
}

pub fn read_config() -> Config {
    let c = Config::default();
    
    return c;
}