use load_config::Loader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
}

fn main() {
    // Load config from file
    let file_config = Loader::load_path::<Config>("examples/config.json").unwrap();

    // Load config from environment variable
    unsafe {
        std::env::set_var("APP_CONFIG", "{\"name\": \"John Doe\"}");
    }
    let env_config = Loader::load_env::<Config>("APP_CONFIG").unwrap();

    println!("Config from file: {}", file_config.name);
    println!("Config from env: {}", env_config.name);

    // Load config from environment variable with file link
    unsafe {
        std::env::set_var("APP_CONFIG", "{\"$path\": \"examples/config.json\"}");
    }
    let linked_config = Loader::load_env::<Config>("APP_CONFIG").unwrap();
    println!("Config from env link: {}", linked_config.name);
}
