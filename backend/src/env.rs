use anyhow::Result;
use std::env::var;

pub struct Env {
    pub port: u16,
    pub public_dir: String,
    pub database_file: String,
}

impl Env {
    pub fn load() -> Result<Env> {
        let port = var("PORT").unwrap_or("8080".to_string()).parse()?;
        let public_dir = var("PUBLIC_DIR").unwrap_or("../frontend/dist".to_string());
        let database_file = var("DATABASE_FILE").unwrap_or("./data/database.db".to_string());

        Ok(Env {
            port,
            public_dir,
            database_file,
        })
    }
}
