use load_config::Loader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

fn expected_database() -> Database {
    let database_json = std::fs::read_to_string("tests/database.json").unwrap();
    serde_json::from_str(&database_json).unwrap()
}

#[test]
fn load_config_from_toml_file() {
    let config = Loader::load_path::<Config>("tests/config.toml").unwrap();
    let database = expected_database();

    assert_eq!(config.database.host, database.host);
    assert_eq!(config.database.port, database.port);
    assert_eq!(config.database.username, database.username);
    assert_eq!(config.database.password, database.password);
    assert_eq!(config.database.database, database.database);
}

#[test]
fn load_config_from_yaml_file() {
    let config = Loader::load_path::<Config>("tests/config.yaml").unwrap();
    let database = expected_database();

    assert_eq!(config.database.host, database.host);
    assert_eq!(config.database.port, database.port);
    assert_eq!(config.database.username, database.username);
    assert_eq!(config.database.password, database.password);
    assert_eq!(config.database.database, database.database);
}
