use anyhow::Result;
use serde::{Deserialize, Serialize};

pub(crate) trait FromStringKeyValue: Sized {
    fn from_iter(iter: &mut impl Iterator<Item = (String, String)>) -> Result<Self>;
}

pub(crate) trait FromEnv: Sized {
    fn from_env() -> Result<Self>;
}

impl<T: FromStringKeyValue> FromEnv for T {
    fn from_env() -> Result<Self> {
        Self::from_iter(&mut std::env::vars())
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) database_authorization: DatabaseAuthorizationInfo,
    pub(crate) ports: ServicePorts,
}

impl FromStringKeyValue for Config {
    fn from_iter(iter: &mut impl Iterator<Item = (String, String)>) -> Result<Self> {
        Ok(Self {
            database_authorization: DatabaseAuthorizationInfo::from_iter(iter)?,
            ports: ServicePorts::from_iter(iter)?,
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

impl FromStringKeyValue for DatabaseAuthorizationInfo {
    fn from_iter(iter: &mut impl Iterator<Item = (String, String)>) -> Result<Self> {
        Ok(envy::prefixed("DB").from_iter(iter)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct ServicePorts {
    pub(crate) http: Port,
}

impl FromStringKeyValue for ServicePorts {
    fn from_iter(iter: &mut impl Iterator<Item = (String, String)>) -> Result<Self> {
        Ok(envy::prefixed("PORT").from_iter(iter)?)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Port(pub(crate) u16);
