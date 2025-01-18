use std::sync::Arc;

use domain::{
    model::content_model::ContentModel,
    repository::content_model::{CreateContentModel, MockContentModelRepository},
};

use crate::usecase::content_model::ContentModelUsecase;

#[rstest::fixture]
fn content_model() -> ContentModel {
    ContentModel::try_new("id".into(), "name".into(), "api_identifier".into(), None).unwrap()
}

#[rstest::fixture]
fn create_content_model() -> CreateContentModel {
    CreateContentModel::new("name".into(), "api_identifier".into(), None)
}

#[tokio::test]
#[rstest::rstest]
async fn get_success(content_model: ContentModel) {
    let mut mock = MockContentModelRepository::new();
    mock.expect_get()
        .returning(move || Ok(vec![content_model.clone()]));
    let usecase = ContentModelUsecase::new(Arc::new(mock));
    let result = usecase.get().await;

    assert_eq!(result.is_ok(), true);
}

#[tokio::test]
#[rstest::rstest]
#[case::simple(CreateContentModel::new("name".into(), "api_identifier".into(), None)
)]
async fn create_success(#[case] create_content_model: CreateContentModel) {
    let mut mock = MockContentModelRepository::new();
    mock.expect_create().returning(move |x| {
        ContentModel::try_new("id".into(), x.name, x.api_identifier, x.description)
    });
    let usecase = ContentModelUsecase::new(Arc::new(mock));
    let result = usecase.create(create_content_model).await;

    assert_eq!(result.is_ok(), true);
}
