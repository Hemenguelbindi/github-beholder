#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
}


impl Config {
    pub fn init() -> Config{
        let token = std::env::var("TOKEN").expect("TOKEN not set");

        Config{
            token,
        }
    }
}