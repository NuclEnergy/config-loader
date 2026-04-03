use load_config::Loader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EnvChainConfig {
    service: EnvChainService,
}

#[derive(Debug, Deserialize)]
struct EnvChainService {
    name: String,
    database: EnvChainDatabase,
}

#[derive(Debug, Deserialize)]
struct EnvChainDatabase {
    host: String,
    port: u16,
    credentials: EnvChainCredentials,
}

#[derive(Debug, Deserialize)]
struct EnvChainCredentials {
    username: String,
    password: String,
}

#[test]
fn load_config_from_multi_level_nested_files_with_deep_env() {
    unsafe {
        std::env::set_var(
            "load_config_DB_CREDENTIALS",
            r#"{"username":"deep-user","password":"deep-password"}"#,
        );
    }

    let config = Loader::load_path::<EnvChainConfig>("tests/nested/env_root.json").unwrap();

    assert_eq!(config.service.name, "orders-api-env");
    assert_eq!(config.service.database.host, "127.0.0.1");
    assert_eq!(config.service.database.port, 5432);
    assert_eq!(config.service.database.credentials.username, "deep-user");
    assert_eq!(
        config.service.database.credentials.password,
        "deep-password"
    );
}

#[test]
fn deep_nested_missing_file_error_propagates() {
    let err = Loader::load_path::<EnvChainConfig>("tests/nested/missing_root.json").unwrap_err();

    let msg = err.to_string();

    assert!(
        msg.contains("failed to read")
            && msg.contains("tests/nested/level1/level2/level3/missing-credentials.json"),
        "unexpected error message: {msg}"
    );
}

#[test]
fn yaml_complex_mapping_key_returns_error() {
    let err = Loader::load_value_from_path("tests/yaml_complex_key.yaml").unwrap_err();

    let msg = err.to_string();

    assert!(
        msg.contains("complex YAML mapping keys are not supported"),
        "unexpected error message: {msg}"
    );
}

#[test]
fn yaml_alias_returns_error() {
    let err = Loader::load_value_from_path("tests/yaml_alias.yaml").unwrap_err();

    let msg = err.to_string();

    assert!(
        msg.contains("failed to parse YAML") && msg.contains("unknown anchor"),
        "unexpected error message: {msg}"
    );
}

#[test]
fn yaml_special_path_non_string_returns_error() {
    let err = Loader::load_value_from_path("tests/yaml_special_path_non_string.yaml").unwrap_err();

    let msg = err.to_string();

    assert!(
        msg.contains("special member $path expects a string value"),
        "unexpected error message: {msg}"
    );
}

#[test]
fn yaml_special_env_non_string_returns_error() {
    let err = Loader::load_value_from_path("tests/yaml_special_env_non_string.yaml").unwrap_err();

    let msg = err.to_string();

    assert!(
        msg.contains("special member $env expects a string value"),
        "unexpected error message: {msg}"
    );
}
