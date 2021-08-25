use std::env;

pub struct Config {
    pub db_host: String,
    pub redis_host: String,
}

pub fn get_config() -> Config {
    let db_host;
    match env::var("DB_HOST") {
        Ok(val) => db_host = val,
        Err(_e) => db_host = "db".to_string(),
    }

    let redis_host;
    match env::var("REDIS_HOST") {
        Ok(val) => redis_host = val,
        Err(_e) => redis_host = "redis".to_string(),
    }

    return Config {
        db_host: db_host,
        redis_host: redis_host,
    };
}