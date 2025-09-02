use crate::parser::ServerCmd;
use anyhow::Result;
use ddns_core::{
    ddns::{auth_service_server::AuthServiceServer, hello_service_server::HelloServiceServer},
    DnsProvider,
};
use tokio::fs;
use tonic::transport::{Identity, ServerTlsConfig};

mod grpc;
mod parser;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    cli().await?;
    Ok(())
}

pub(crate) async fn cli() -> Result<()> {
    let args = parser::parse_args();
    match args.cmd {
        Some(parser::Cmd::Server(server)) => start_grpc_server(server).await?,
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
    // List Zone
    // env var => DUACODIE_DDNS_CLIENT_API_TOKEN or DUACODIE_DDNS_SERVER_API_TOKEN
    let token = std::env::var("DUACODIE_DDNS_SERVER_API_TOKEN")
        .expect("DUACODIE_DDNS_SERVER_API_TOKEN must be set");
    let provider = ddns_core::provider::CloudflareProvider::new(token, None, None);
    let zones = provider.list_zone().await?;
    println!("{zones:#?}");
    Ok(())
}

async fn load_identity(cert_path: &str, key_path: &str) -> Result<Identity> {
    let cert = fs::read(cert_path).await?;
    let key = fs::read(key_path).await?;
    Ok(Identity::from_pem(cert, key))
}

async fn start_grpc_server(serve: ServerCmd) -> Result<()> {
    let addr = format!("{}:{}", serve.host, serve.port).parse()?;
    let identity =
        load_identity("DDNS-Server/certs/fullchain.pem", "DDNS-Server/certs/privkey.pem").await?;
    let tls = ServerTlsConfig::new().identity(identity);
    let auth_service = grpc::MyAuthService::default();
    let hello_service = grpc::MyHelloService::default();
    tonic::transport::Server::builder()
        .tls_config(tls)?
        .add_service(AuthServiceServer::new(auth_service))
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;
    Ok(())
}
