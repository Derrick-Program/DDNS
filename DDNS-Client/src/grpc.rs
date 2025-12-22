use anyhow::{Context, Result};
use ddns_core::ddns::{
    auth_service_client::AuthServiceClient, hello_service_client::HelloServiceClient, HelloRequest,
    LoginRequest,
};
use tonic::transport::{Channel, ClientTlsConfig};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct GrpcClient {
    auth_client:  AuthServiceClient<Channel>,
    hello_client: HelloServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(server: &str) -> Result<Self> {
        let tls = ClientTlsConfig::new().with_enabled_roots().domain_name("ddns.duacodie.com");
        let channel = Channel::from_shared(server.to_string())
            .with_context(|| format!("failed to create channel for {server}"))?
            .tls_config(tls)
            .with_context(|| "failed to create TLS config")?
            .connect()
            .await
            .with_context(|| format!("failed to connect to {server}"))?;
        let auth_client = AuthServiceClient::new(channel.clone());
        let hello_client = HelloServiceClient::new(channel);
        Ok(Self { auth_client, hello_client })
    }
    pub async fn login(
        &mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<String> {
        let request = tonic::Request::new(LoginRequest {
            username: username.into(),
            password: password.into(),
        });
        let response = self.auth_client.login(request).await?.into_inner();
        if response.success {
            Ok(response.access_token)
        } else {
            Err(anyhow::anyhow!("Login failed"))
        }
    }
    pub async fn hello(&mut self, name: impl Into<String>) -> Result<String> {
        let request = tonic::Request::new(HelloRequest { name: name.into() });
        let response = self.hello_client.say_hello(request).await?;
        Ok(response.into_inner().message)
    }
}
