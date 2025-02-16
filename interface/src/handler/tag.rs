
use super::error::AppResult;

#[derive(utoipa::IntoParams)]
pub struct GetTagQuery {}

#[utoipa::path(
    get,
    path = "/tags",
    params(GetTagQuery), 
    responses(
        (status = 200, description = "Get tag success", body = ())
    ),
    tag = "tags",
)]
pub fn get_tag() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::IntoParams)]
pub struct FindTagQuery {}

#[utoipa::path(
    get,
    path = "/tags/{id}",
    params(FindTagQuery),
    responses(
        (status = 200, description = "Find tag success", body = ())
    ),
    tag = "tags",
)]
pub fn find_tag() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct CreateTagJson {}

#[utoipa::path(
    post,
    path = "/tags",
    request_body = CreateTagJson,
    responses(
        (status = 200, description = "Create tag success", body = ())
    ),
    tag = "tags",
)]
pub fn create_tag() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct UpdateTagJson {}

#[utoipa::path(
    put,
    path = "/tags/{id}",
    request_body = UpdateTagJson,
    responses(
        (status = 200, description = "Update tag success", body = ())
    ),
    tag = "tags",
)]
pub fn update_tag() -> AppResult<()> {
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/tags/{id}",
    responses(
        (status = 200, description = "Delete tag success")
    ),
    tag = "tags",
)]
pub fn delete_tag() -> AppResult<()> {
    Ok(())
}
