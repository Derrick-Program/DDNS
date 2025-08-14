use crate::cli::cli;
use anyhow::Result;
use ddns_core::get_public_ip;

mod cli;
mod grpc;
mod parser;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    cli()?;
    // let public_ip = get_public_ip().await?;
    // println!("Public IP: {}", public_ip);
    // let server_address = "http://[::1]:50051";
    // let mut grpc_client = grpc::GrpcClient::connect(server_address).await?;
    // match grpc_client.login("admin", "password").await {
    //     Ok(token) => {
    //         println!("Login successful: {}", token);
    //     }
    //     Err(e) => {
    //         eprintln!("Login failed");
    //         std::process::exit(1);
    //     }
    // }
    // let msg = grpc_client.hello("derrick").await?;
    // println!("GRPC Response: {:?}", msg);
    Ok(())
}
