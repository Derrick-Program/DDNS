#[derive(thiserror::Error, Debug)]
pub enum DdnsError {
    #[error("Server error: {0}")]
    ServerError(#[from] DdnsServerError),
    #[error("Client error: {0}")]
    ClientError(#[from] DdnsClientError),
}

#[derive(thiserror::Error, Debug)]
pub enum DdnsServerError {
    #[error("解析命令行參數時出錯: {0}")]
    CliParseError(String),
    #[error("啟動伺服器時出錯: {0}")]
    ServerStartError(String),
    #[error("使用者管理錯誤: {0}")]
    UserManagementError(String),
    #[error("提供者錯誤: {0}")]
    DnsProviderError(#[from] DnsError),
}

#[derive(thiserror::Error, Debug)]
pub enum DdnsClientError {
    #[error("客戶端錯誤: {0}")]
    ClientError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum DnsError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("resource not found")]
    NotFound,
    #[error("rate limited")]
    RateLimited,
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("provider error: {0}")]
    Provider(String),
    #[error("transient error: {0}")]
    Transient(String),
}
