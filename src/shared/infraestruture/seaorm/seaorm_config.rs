pub struct SeaOrmConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub schema: Option<String>,
}
