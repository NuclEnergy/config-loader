use load_config::Loader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    service: Service,
}

#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
    host: String,
    port: u16,
    credentials: Credentials,
}

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[test]
fn load_config_from_multi_level_nested_files() {
    let config = Loader::load_path::<AppConfig>("tests/nested/root.json").unwrap();

    assert_eq!(config.service.name, "orders-api");
    assert_eq!(config.service.database.host, "127.0.0.1");
    assert_eq!(config.service.database.port, 5432);
    assert_eq!(config.service.database.credentials.username, "postgres");
    assert_eq!(
        config.service.database.credentials.password,
        "postgres_password"
    );
}
