use crate::parser;
use anyhow::Result;

pub(crate) fn cli() -> Result<()> {
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
                println!("{}", format!("Version: {}", env!("CARGO_PKG_VERSION")));
            } else if args.debug {
                debug()?;
            } else {
                eprintln!("No command provided. Use --help for usage information.");
            }
        }
    }
    Ok(())
}

fn debug() -> Result<()> {
    println!("Debug mode is enabled. No server will be started.");
    Ok(())
}
