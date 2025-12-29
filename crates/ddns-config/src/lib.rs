use std::env;
use dotenvy::from_filename;

pub fn from_env() {
    let app_env = env::var("DDNS_APP_ENV").unwrap_or_else(|_| "prod".to_string());
    let env_file = match app_env.as_str() {
        "prod" => ".env.prod",
        "dev" => ".env.dev",
        "test" => ".env.test",
        _ => ".env",
    };
    match from_filename(env_file) {
        Ok(_) => println!("已載入設定檔: {}", env_file),
        Err(e) => println!("警告: 無法載入 {}: {}", env_file, e),
    }
}
