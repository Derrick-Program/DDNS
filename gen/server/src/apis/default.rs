use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthLoginResponse {
    /// The request has succeeded.
    Status200_TheRequestHasSucceeded
    (models::AuthLogin200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthLogoutResponse {
    /// The request has succeeded.
    Status200_TheRequestHasSucceeded
    (models::ErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthRefreshResponse {
    /// The request has succeeded.
    Status200_TheRequestHasSucceeded
    (models::AuthLogin200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RecordsListResponse {
    /// The request has succeeded.
    Status200_TheRequestHasSucceeded
    (models::RecordsList200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatesUpdateResponse {
    /// The request has succeeded.
    Status200_TheRequestHasSucceeded
    (models::UpdatesUpdate200Response)
}




/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// AuthLogin - POST /v1/auth/login
    async fn auth_login(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::LoginRequest,
    ) -> Result<AuthLoginResponse, E>;

    /// AuthLogout - POST /v1/auth/logout
    async fn auth_logout(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::LogoutRequest,
    ) -> Result<AuthLogoutResponse, E>;

    /// AuthRefresh - POST /v1/auth/refresh
    async fn auth_refresh(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::RefreshRequest,
    ) -> Result<AuthRefreshResponse, E>;

    /// RecordsList - GET /v1/records
    async fn records_list(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<RecordsListResponse, E>;

    /// UpdatesUpdate - POST /v1/records/{recordId}:update
    async fn updates_update(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::UpdatesUpdatePathParams,
            body: &models::UpdateRecordRequest,
    ) -> Result<UpdatesUpdateResponse, E>;
}
