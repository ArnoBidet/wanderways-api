use std::env;

pub struct EnvVar {
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
    pub db_host: String,
}

pub fn env_parser() -> EnvVar {
    // Usage of unwrap() because we want it to fail if not found, no need for grace
    EnvVar {
        db_user: env::var("DB_USER").unwrap(),
        db_pass: env::var("DB_PASS").unwrap(),
        db_name: env::var("DB_NAME").unwrap(),
        db_host: env::var("DB_HOST").unwrap(),
    }
}