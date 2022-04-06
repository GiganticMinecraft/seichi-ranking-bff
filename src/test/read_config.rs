use crate::Config;
use crate::config::FromStringKeyValue;

#[test]
fn read_config_from_iterator() {
    let setting = [
        ("HTTP_PORT".to_string(), "12345".to_string()),
        ("DB_HOST".to_string(), "example.com".to_string()),
        ("DB_PORT".to_string(), "3307".to_string()),
        ("DB_USER".to_string(), "bff".to_string()),
        ("DB_PASSWORD".to_string(), "$tr0ngpAssw0rd".to_string()),
    ];

    Config::from_iter(&mut setting.into_iter()).unwrap();

}