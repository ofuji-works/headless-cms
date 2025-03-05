use utoipa::OpenApi;

use crate::handler;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::category::get_categories,
        handler::category::create_category,
        handler::category::update_category,
        handler::category::delete_category,

        handler::content::get_contents,
        handler::content::create_content,
        handler::content::update_content,
        handler::content::delete_content,

        handler::tag::get_tags,
        handler::tag::find_tag,
        handler::tag::create_tag,
        handler::tag::update_tag,
        handler::tag::delete_tag,
    ),
    components(schemas(
        domain::model::category::Category,
        handler::category::GetCategoryQuery,
        handler::category::CreateCategoryJson,
        handler::category::UpdateCategoryJson,

        domain::model::content::Content,
        handler::content::GetContentQuery,
        handler::content::CreateContentJson,
        handler::content::UpdateContentJson,

        domain::model::tag::Tag,
        handler::tag::GetTagQuery,
        handler::tag::CreateTagJson,
        handler::tag::UpdateTagJson,
    )),
    tags(
        (name = "categories"),
        (name = "contents"),
        (name = "tags"),
    ),
)]
pub struct ApiDoc;
