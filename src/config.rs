use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv().ok();

        AppConfig {
            port: env::var("APP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("APP_PORT must be a number"),
        }
    }
}
