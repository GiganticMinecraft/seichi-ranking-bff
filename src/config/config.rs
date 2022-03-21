use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) database_authorization: DatabaseAuthorizationInfo,
    pub(crate) ports: ServicePorts,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseAuthorizationInfo {
    pub(crate) host: String,
    pub(crate) port: Port,
    pub(crate) user: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Debug)]
pub struct ServicePorts {
    pub(crate) https: Port,
    pub(crate) http: Port,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Port(pub(crate) u16);
