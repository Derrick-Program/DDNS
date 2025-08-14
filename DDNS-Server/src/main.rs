use crate::cli::cli;
use ddns_core::{
    ddns::{auth_service_server::AuthServiceServer, hello_service_server::HelloServiceServer},
    tonic,
};

mod cli;
mod grpc;
mod parser;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    cli()?;
    // let addr = "[::1]:50051".parse()?;
    // let auth_service = crate::grpc::MyAuthService::default();
    // let hello_service = crate::grpc::MyHelloService::default();
    // tonic::transport::Server::builder()
    //     .add_service(AuthServiceServer::new(auth_service))
    //     .add_service(HelloServiceServer::new(hello_service))
    //     .serve(addr)
    //     .await?;
    Ok(())
}
