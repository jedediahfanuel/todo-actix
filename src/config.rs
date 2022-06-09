use dotenv_codegen::dotenv;

// use serde::Deserialize;
// use config::ConfigError;

// #[derive(Deserialize)]
// pub struct ServerConfig {
//     pub host: String,
//     pub port: i32
// }

// #[derive(Deserialize)]
// pub struct Config {
//     pub server: ServerConfig
// }

// impl Config {
//     pub fn from_env() -> Result<Self, ConfigError> {
//         let mut cfg = config::Config::new();
//         cfg.merge(config::Environtment::new())?;
//         cfg.try_into()
//     }
// }

pub struct ServerConfig {
    pub host: String,
    pub port: String
}

pub struct Config {
    pub server: ServerConfig
}

impl Config {
    pub fn from_env() -> Self {
        let host_env = dotenv!("SERVER.HOST");
        let port_env = dotenv!("SERVER.PORT");
        Self {
            server: ServerConfig {
                host: host_env.to_string(),
                port: port_env.to_string()
            }
        }
    }
}