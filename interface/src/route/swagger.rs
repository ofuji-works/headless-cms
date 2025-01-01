use utoipa::OpenApi;

use crate::handler;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::content_model::get_content_models,
        handler::content_model::create_content_model,
        handler::content_model::update_content_model,
        handler::content_model::delete_content_model,

        handler::content::get_contents,
        handler::content::create_content,
        handler::content::update_content,
        handler::content::delete_content,
    ),
    components(schemas(
        domain::model::content_model::ContentModel,
        handler::content_model::GetContentModelQuery,
        handler::content_model::UpdateContentModelJson,

        domain::model::content::Content,
        handler::content::GetContentQuery,
        handler::content::UpdateContentJson,
    )),
    tags(
        (name = "content_models"),
        (name = "contents"),
    ),
)]
pub struct ApiDoc;
