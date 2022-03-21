use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub(crate) database_authorization: DatabaseAuthorizationInfo,
    pub(crate) ports: ServicePorts,
}

#[derive(Deserialize)]
pub struct DatabaseAuthorizationInfo {
    pub(crate) host: String,
    pub(crate) port: Port,
    pub(crate) user: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub struct ServicePorts {
    pub(crate) https: Port,
    pub(crate) http: Port,
}

#[derive(Serialize, Deserialize)]
pub struct Port(pub(crate) u16);
