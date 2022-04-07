use anyhow::Result;
use envy::Error;
use serde::{Deserialize, Serialize};

pub trait FromEnvLikeKeyValuePairs: Sized {
    fn from_iter(iter: impl Iterator<Item = (String, String)> + Clone) -> Result<Self, Error>;
}

pub trait FromEnv: Sized {
    fn from_env() -> Result<Self, Error>;
}

impl<T: FromEnvLikeKeyValuePairs> FromEnv for T {
    fn from_env() -> Result<Self, Error> {
        // std::env::Vars is !Clone
        Self::from_iter(std::env::vars().collect::<Vec<_>>().into_iter())
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) database_authorization: DatabaseAuthorizationInfo,
    pub(crate) http_config: HttpConfig,
}

impl FromEnvLikeKeyValuePairs for Config {
    fn from_iter(iter: impl Iterator<Item = (String, String)> + Clone) -> Result<Self, Error> {
        Ok(Self {
            database_authorization: DatabaseAuthorizationInfo::from_iter(iter.clone())?,
            http_config: HttpConfig::from_iter(iter)?,
        })
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DatabaseAuthorizationInfo {
    pub(crate) host: String,
    pub(crate) port: Port,
    pub(crate) user: String,
    pub(crate) password: String,
}

impl FromEnvLikeKeyValuePairs for DatabaseAuthorizationInfo {
    fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Result<Self, Error> {
        envy::prefixed("DB_").from_iter(iter)
    }
}

#[derive(Deserialize, Debug)]
pub struct HttpConfig {
    pub port: Port,
}

impl FromEnvLikeKeyValuePairs for HttpConfig {
    fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Result<Self, Error> {
        envy::prefixed("HTTP_").from_iter(iter)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Port(pub(crate) u16);

#[cfg(test)]
mod test {
    use crate::config::FromEnvLikeKeyValuePairs;
    use crate::Config;

    #[test]
    fn read_config_from_iterator() {
        let setting = [
            ("HTTP_PORT".to_string(), "12345".to_string()),
            ("DB_HOST".to_string(), "example.com".to_string()),
            ("DB_PORT".to_string(), "3307".to_string()),
            ("DB_USER".to_string(), "bff".to_string()),
            ("DB_PASSWORD".to_string(), "$tr0ngpAssw0rd".to_string()),
        ];

        Config::from_iter(setting.into_iter()).unwrap();
    }
}
