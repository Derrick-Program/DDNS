use anyhow::Result;
mod parser;
mod token;
pub async fn cli() -> Result<()> {
    let args = parser::parse_args();
    match args.cmd {
        // Some(parser::Cmd::Server(server)) => start_grpc_server(server).await?,
        Some(parser::Cmd::Server(server)) => {
            dbg!(server);
            return Ok(());
        }
        Some(parser::Cmd::User(user)) => {
            return match user.action {
                parser::UserAction::Add(add) => {
                    dbg!(add);
                    Ok(())
                }
                parser::UserAction::Rm(rm) => {
                    dbg!(rm);
                    Ok(())
                }
                parser::UserAction::List(list) => {
                    dbg!(list);
                    Ok(())
                }
            };
        }
        None => {
            if args.version {
                println!("{}", format_args!("Version: {}", env!("CARGO_PKG_VERSION")));
            } else if args.debug && cfg!(debug_assertions) {
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
    // println!("{zones:#?}");
    Ok(())
}
