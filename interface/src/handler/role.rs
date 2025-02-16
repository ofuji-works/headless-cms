use super::error::AppResult;

#[derive(utoipa::IntoParams)]
pub struct GetRoleQuery {}

#[utoipa::path(
    get,
    path = "/roles",
    params(GetRoleQuery),
    responses(
        (status = 200, description = "Get role success", body = ())
    ),
    tag = "roles",
)]
pub fn get_role() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::IntoParams)]
pub struct FindRoleQuery {}

#[utoipa::path(
    get,
    path = "/roles",
    params(FindRoleQuery),
    responses(
        (status = 200, description = "Find role success", body = ())
    ),
    tag = "roles",
)]
pub fn find_role() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct CreateRoleJson {}

#[utoipa::path(
    post,
    path = "/roles",
    request_body = CreateRoleJson,
    responses(
        (status = 200, description = "Create role success", body = ())
    ),
    tag = "roles",
)]
pub fn create_role() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct UpdateRoleJson {}

#[utoipa::path(
    put,
    path = "/roles",
    request_body = UpdateRoleJson,
    responses(
        (status = 200, description = "Update role success", body = ())
    ),
    tag = "roles",
)]
pub fn update_role() -> AppResult<()> {
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/roles",
    responses(
        (status = 200, description = "Delete role success", body = ())
    ),
    tag = "roles",
)]
pub fn delete_role() -> AppResult<()> {
    Ok(())
}
