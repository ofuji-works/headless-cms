use crate::handler::error::AppResult;

#[derive(utoipa::ToSchema)]
pub struct SignUpJson {}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = SignUpJson,
    responses(
        (status = 200, description = "Signup success", body = ())
    ),
    tag = "auth",
)]
pub async fn signup() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct VerifyJson {}

#[utoipa::path(
    post,
    path = "/auth/verify",
    request_body = VerifyJson,
    responses(
        (status = 200, description = "Verify success", body = ())
    ),
    tag = "auth",
)]
pub async fn verify() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct SignInJson {}

#[utoipa::path(
    post,
    path = "/auth/signin",
    request_body = SignInJson,
    responses(
        (status = 200, description = "Signin success", body = ())
    ),
    tag = "auth",
)]
pub async fn signin() -> AppResult<()> {
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/auth/signout",
    responses(
        (status = 200, description = "Signout success", body = ())
    ),
    tag = "auth",
)]
pub async fn signout() -> AppResult<()> {
    Ok(())
}
