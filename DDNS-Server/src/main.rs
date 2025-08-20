use crate::cli::cli;

mod cli;
mod grpc;
mod parser;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    cli().await?;
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
