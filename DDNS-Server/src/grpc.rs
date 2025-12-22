use crate::token::TokenService;
use ddns_core::ddns::{
    auth_service_server::AuthService, hello_service_server::HelloService, HelloRequest, HelloResponse, LoginRequest, LoginResponse,
    LogoutRequest, LogoutResponse,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyAuthService {
    token_svc: TokenService,
}

#[tonic::async_trait]
impl AuthService for MyAuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        // TODO: 從 DB 找 user，argon2 驗證密碼
        if req.username != "admin" || req.password != "1234" {
            return Err(Status::unauthenticated("invalid credentials"));
        }
        let (tk, _) = self.token_svc.issue(&req.username).map_err(to_status)?;
        Ok(Response::new(LoginResponse { success: true, access_token: tk }))
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        // 如果要支援強制登出，這裡把 token 加到 denylist
        Ok(Response::new(LogoutResponse { success: true }))
    }
}

fn to_status(e: anyhow::Error) -> Status {
    Status::internal(e.to_string())
}

#[derive(Debug, Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner();
        let response = HelloResponse { message: format!("Hello, {}!", name.name) };
        Ok(Response::new(response))
    }
}
