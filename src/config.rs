use std::{env, sync::OnceLock};

use crate::Result;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub DIRECTORY: Option<String>,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DIRECTORY: get_env("--directory"),
        })
    }
}

fn get_env(flag: &'static str) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    args.iter()
        .position(|arg| arg == flag)
        .and_then(|index| args.get(index + 1))
        .map(|st| st.to_string())
}
