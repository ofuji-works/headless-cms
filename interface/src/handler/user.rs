use super::error::AppResult;

#[derive(utoipa::IntoParams)]
pub struct GetUserQuery {}

#[utoipa::path(
    get,
    path = "/users",
    params(GetUserQuery), 
    responses(
        (status = 200, description = "Get user success", body = ())
    ),
    tag = "users",
)]
pub fn get_user() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::IntoParams)]
pub struct FindUserQuery {}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(FindUserQuery), 
    responses(
        (status = 200, description = "Find user success", body = ())
    ),
    tag = "users",
)]
pub fn find_user() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct UpdateUserJson {}

#[utoipa::path(
    put,
    path = "/users/{id}",
    request_body = UpdateUserJson,
    responses(
        (status = 200, description = "Update user success", body = ())
    ),
    tag = "users",
)]
pub fn update_user() -> AppResult<()> {
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Delete user success", body = ())
    ),
    tag = "users",
)]
pub fn delete_user() -> AppResult<()> {
    Ok(())
}
