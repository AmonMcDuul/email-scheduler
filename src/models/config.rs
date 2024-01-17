#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    pub credentials: Credentials,
}

#[derive(Debug, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
