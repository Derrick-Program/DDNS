// use argh::FromArgs;

// #[derive(FromArgs, Debug)]
// /// Duacodie DDNS 客戶端
// pub(crate) struct Args {
//     // /// 詳細輸出
//     // #[argh(switch, short = 'v')]
//     // verbose: bool,
//     // #[argh(subcommand)]
//     // pub(crate) auth: Auth,
//     #[argh(subcommand)]
//     pub(crate) cmd: Cmd,
// }

// #[derive(FromArgs, Debug)]
// #[argh(subcommand)]
// pub(crate) enum Cmd {
//     Auth(Auth),
// }

// #[derive(FromArgs, Debug)]
// #[argh(subcommand, name = "auth")]
// /// 認證相關操作
// pub(crate) struct Auth {
//     #[argh(subcommand)]
//     pub(crate) cmd: AuthCmd,
// }

// #[derive(FromArgs, Debug)]
// #[argh(subcommand)]
// pub(crate) enum AuthCmd {
//     Login(Login),
//     Logout(Logout),
// }

// #[derive(FromArgs, Debug)]
// #[argh(subcommand, name = "login")]
// /// 登入到 DDNS 伺服器
// pub(crate) struct Login {
//     #[argh(option, short = 'u')]
//     /// 使用者名稱
//     pub(crate) username:        String,
//     #[argh(switch)]
//     /// 啟用互動式密碼輸入（終端遮罩顯示）
//     pub(crate) password_prompt: bool,
//     /// 從標準輸入讀取密碼（適合自動化）
//     #[argh(switch, short = 'i')]
//     pub(crate) password_stdin:  bool,
//     #[argh(option, short = 'p')]
//     /// (不建議）直接傳入密碼；會出現在 shell 歷史/ps
//     pub(crate) password:        Option<String>,
// }
// #[derive(FromArgs, Debug)]
// #[argh(subcommand, name = "logout")]
// /// 登出 DDNS 伺服器
// pub struct Logout {}
// pub(crate) fn parse_args() -> Args {
//     argh::from_env()
// }


use clap::{Args, Parser, Subcommand};

/// Duacodie DDNS 客戶端
#[derive(Parser, Debug)]
#[command(name = "ddns-client", version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Cmd {
    /// 認證相關操作
    Auth(Auth),
}

#[derive(Args, Debug)]
pub(crate) struct Auth {
    #[command(subcommand)]
    pub(crate) cmd: AuthCmd,
}

#[derive(Subcommand, Debug)]
pub(crate) enum AuthCmd {
    /// 登入到 DDNS 伺服器
    Login(Login),
    /// 登出 DDNS 伺服器
    Logout(Logout),
}

#[derive(Args, Debug)]
pub(crate) struct Login {
    /// 使用者名稱
    #[arg(short = 'u', long)]
    pub(crate) username: String,

    /// 啟用互動式密碼輸入（終端遮罩顯示）
    ///
    /// 這三個 password* 參數是互斥的：只能選一個。
    #[arg(long, conflicts_with_all = ["password_stdin", "password"])]
    pub(crate) password_prompt: bool,

    /// 從標準輸入讀取密碼（適合自動化）
    #[arg(short = 'i', long, conflicts_with_all = ["password_prompt", "password"])]
    pub(crate) password_stdin: bool,

    /// （不建議）直接傳入密碼；會出現在 shell 歷史/ps
    #[arg(short = 'p', long, value_name = "PASSWORD", conflicts_with_all = ["password_prompt", "password_stdin"])]
    pub(crate) password: Option<String>,

    /// （可選）也支援用環境變數帶密碼（更適合 CI）
    /// - 若你不想提供 env，就刪掉這個欄位即可
    #[arg(long, env = "DDNS_PASSWORD", hide_env_values = true, value_name = "PASSWORD",
          conflicts_with_all = ["password_prompt", "password_stdin", "password"])]
    pub(crate) password_env: Option<String>,
}

#[derive(Args, Debug)]
pub struct Logout {}

pub(crate) fn parse_args() -> Cli {
    Cli::parse()
}
