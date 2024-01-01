use spaceapi_dezentrale_server::{config::SpaceConfig, serve};

#[rocket::main]
async fn main() {
    env_logger::init();
    let config_file = std::env::var("CONFIG_FILE").unwrap_or("config.yml".to_string());
    let config = SpaceConfig::load(config_file).expect("Invalid config");
    let _ = serve(config).await.launch().await.expect("Can't start server");
}
