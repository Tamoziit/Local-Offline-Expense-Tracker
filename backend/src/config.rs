#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be defined in .env");
        let host = std::env::var("HOST").expect("HOST must be defined in .env");
        let port = std::env::var("PORT")
            .expect("PORT must be defined in .env")
            .parse::<u16>()
            .expect("PORT must be a valid u16");

        Config {
            database_url,
            host,
            port,
        }
    }
}
