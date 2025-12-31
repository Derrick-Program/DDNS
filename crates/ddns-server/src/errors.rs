#[derive(thiserror::Error, Debug)]
pub enum DdnsServerError {
    #[error("解析命令行參數時出錯: {0}")]
    CliParseError(String),
    #[error("啟動伺服器時出錯: {0}")]
    ServerStartError(String),
    #[error("使用者管理錯誤: {0}")]
    UserManagementError(String),
}
