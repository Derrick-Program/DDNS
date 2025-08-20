use crate::parser;
use anyhow::Result;
use ddns_core::DnsProvider;

pub(crate) async fn cli() -> Result<()> {
    let args = parser::parse_args();
    match args.cmd {
        Some(parser::Cmd::Server(server)) => {}
        Some(parser::Cmd::User(user)) => match user.action {
            parser::UserAction::Add(add) => {}
            parser::UserAction::Rm(rm) => {}
            parser::UserAction::List(_) => {}
        },
        None => {
            if args.version {
                println!("{}", format_args!("Version: {}", env!("CARGO_PKG_VERSION")));
            } else if args.debug {
                debug().await?;
            } else {
                eprintln!("沒有提供指令。 使用--help查看可用指令。");
            }
        }
    }
    Ok(())
}

async fn debug() -> Result<()> {
    println!("Debug mode is enabled. No server will be started.");
    // List Zone
    // env var => DUACODIE_DDNS_CLIENT_API_TOKEN or DUACODIE_DDNS_SERVER_API_TOKEN
    let token = std::env::var("DUACODIE_DDNS_SERVER_API_TOKEN").expect("CF_API_TOKEN must be set");
    let provider = ddns_core::provider::CloudflareProvider::new(token, None, None);
    let zones = provider.list_zone().await?;
    println!("{:#?}", zones);
    Ok(())
}
