use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Duacodie DDNS 客戶端
pub(crate) struct Args {
    // /// 詳細輸出
    // #[argh(switch, short = 'v')]
    // verbose: bool,
    // #[argh(subcommand)]
    // pub(crate) auth: Auth,
    #[argh(subcommand)]
    pub(crate) cmd: Cmd,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub(crate) enum Cmd {
    Auth(Auth),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "auth")]
/// 認證相關操作
pub(crate) struct Auth {
    #[argh(subcommand)]
    pub(crate) cmd: AuthCmd,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub(crate) enum AuthCmd {
    Login(Login),
    Logout(Logout),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "login")]
/// 登入到 DDNS 伺服器
pub(crate) struct Login {
    #[argh(option, short = 'u')]
    /// 使用者名稱
    pub(crate) username:        String,
    #[argh(switch)]
    /// 啟用互動式密碼輸入（終端遮罩顯示）
    pub(crate) password_prompt: bool,
    /// 從標準輸入讀取密碼（適合自動化）
    #[argh(switch, short = 'i')]
    pub(crate) password_stdin:  bool,
    #[argh(option, short = 'p')]
    /// (不建議）直接傳入密碼；會出現在 shell 歷史/ps
    pub(crate) password:        Option<String>,
}
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "logout")]
/// 登出 DDNS 伺服器
pub struct Logout {}
pub(crate) fn parse_args() -> Args {
    argh::from_env()
}
