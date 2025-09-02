use argh::FromArgs;

/// Duacodie DDNS 伺服端
#[derive(FromArgs, Debug)]
pub struct Cli {
    /// 最外層指令
    #[argh(subcommand)]
    pub cmd:     Option<Cmd>,
    /// 顯示版本資訊
    #[argh(switch, short = 'v')]
    pub version: bool,
    #[argh(switch, short = 'd')]
    /// 測試模式（不啟動伺服器）
    pub debug:   bool,
}

/// 頂層子指令集合
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Cmd {
    User(UserCmd),
    Server(ServerCmd),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "user")]
/// 使用者管理相關操作
pub struct UserCmd {
    #[argh(subcommand)]
    pub action: UserAction,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "server")]
/// 伺服器相關設定
pub struct ServerCmd {
    #[argh(option, short = 'p', default = "50051")]
    /// 伺服器監聽的端口號，預設為 50051
    pub port: u16,
    #[argh(
        option,
        short = 'h',
        default = "String::from(\"127.0.0.1\")",
        description = "伺服器監聽的主機地址，預設為 [127.0.0.1]（localhost）"
    )]
    pub host: String,
}

/// `user` 下的動作們
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum UserAction {
    /// 新增使用者
    Add(UserAdd),
    /// 刪除使用者
    Rm(UserRm),
    /// 範例：列出使用者
    List(UserList),
}

/// `user add` 的參數
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "add")]
/// 添加使用者
pub struct UserAdd {
    #[argh(option, short = 'u')]
    /// 使用者名稱
    pub(crate) username:        String,
    #[argh(switch)]
    /// 啟用互動式密碼輸入 (預設)（終端遮罩顯示）
    pub(crate) password_prompt: bool,
    /// 從標準輸入讀取密碼（適合自動化）
    #[argh(switch, short = 'i')]
    pub(crate) password_stdin:  bool,
    #[argh(option, short = 'p')]
    /// (不建議）直接傳入密碼；會出現在 shell 歷史/ps
    pub(crate) password:        Option<String>,
}

/// `user rm` 的參數
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "rm", description = "Remove a user")]
pub struct UserRm {
    /// 使用者名稱
    #[argh(option, short = 'u')]
    pub username: String,

    /// 強制刪除
    #[argh(switch, short = 'f')]
    pub force: bool,
}

/// `user list` 的參數（示例）
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "list", description = "List users")]
pub struct UserList {
    /// 顯示詳細資訊
    #[argh(switch, short = 'v')]
    pub verbose: bool,
}

pub fn parse_args() -> Cli {
    argh::from_env()
}
