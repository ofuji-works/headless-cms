use super::error::AppResult;

#[derive(utoipa::IntoParams)]
pub struct GetMediaQuery {}

#[utoipa::path(
    get,
    path = "/medias",
    params(GetMediaQuery),
    responses(
        (status = 200, description = "Get media success", body = ())
    ),
    tag = "medias",
)]
pub fn get_media() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::IntoParams)]
pub struct FindMediaQuery {}

#[utoipa::path(
    get,
    path = "/medias/{id}",
    params(FindMediaQuery),
    responses(
        (status = 200, description = "Find media success", body = ())
    ),
    tag = "medias",
)]
pub fn find_media() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct CreateMediaJson {}

#[utoipa::path(
    post,
    path = "/medias",
    request_body = CreateMediaJson,
    responses(
        (status = 200, description = "Create media success", body = ())
    ),
    tag = "medias",
)]
pub fn create_media() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct UpdateMediaJson {}

#[utoipa::path(
    put,
    path = "/medias/{id}",
    request_body = UpdateMediaJson,
    responses(
        (status = 200, description = "Update media success", body = ())
    ),
    tag = "medias",
)]
pub fn update_media() -> AppResult<()> {
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/medias/{id}",
    responses(
        (status = 200, description = "Delete media success", body = ())
    ),
    tag = "medias",
)]
pub fn delete_media() -> AppResult<()> {
    Ok(())
}
