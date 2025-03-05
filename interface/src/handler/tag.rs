use domain::model::tag::Tag;

use crate::handler::error::AppResult;

#[derive(utoipa::IntoParams, utoipa::ToSchema)]
pub struct GetTagQuery {
    #[param(example = 0)]
    pub offset: usize,
    #[param(example = 100)]
    pub limit: usize,
    pub keyword: Option<String>,
}

#[utoipa::path(
    get,
    path = "/tags",
    params(GetTagQuery), 
    responses(
        (status = 200, description = "Get tag success", body = [Tag])
    ),
    tag = "tags",
)]
pub fn get_tags() -> AppResult<()> {
    Ok(())
}

#[utoipa::path(
    get,
    path = "/tags/{id}",
    responses(
        (status = 200, description = "Find tag success", body = Tag)
    ),
    tag = "tags",
)]
pub fn find_tag() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct CreateTagJson {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/tags",
    request_body = CreateTagJson,
    responses(
        (status = 200, description = "Create tag success", body = Tag)
    ),
    tag = "tags",
)]
pub fn create_tag() -> AppResult<()> {
    Ok(())
}

#[derive(utoipa::ToSchema)]
pub struct UpdateTagJson {
    pub name: Option<String>
}

#[utoipa::path(
    put,
    path = "/tags/{id}",
    request_body = UpdateTagJson,
    responses(
        (status = 200, description = "Update tag success", body = Tag)
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
