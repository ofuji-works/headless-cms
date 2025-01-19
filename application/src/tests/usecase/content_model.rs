use std::sync::Arc;

use domain::{
    model::content_model::ContentModel,
    repository::content_model::{
        CreateContentModel, MockContentModelRepository, UpdateContentModel,
    },
};

use crate::usecase::content_model::ContentModelUsecase;

#[rstest::fixture]
fn content_model() -> ContentModel {
    ContentModel::try_new("id".into(), "name".into(), "api_identifier".into(), None).unwrap()
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

#[tokio::test]
#[rstest::rstest]
#[case::simple(UpdateContentModel::new("id".into(), Some("new_name".into()), Some("new_api_identifier".into()), None))]
async fn update_success(
    content_model: ContentModel,
    #[case] update_content_model: UpdateContentModel,
) {
    let mut mock = MockContentModelRepository::new();
    mock.expect_update().returning(move |x| {
        let name = match x.name {
            Some(name) => name,
            None => content_model.name.clone(),
        };
        let api_identifier = match x.api_identifier {
            Some(api_identifier) => api_identifier,
            None => content_model.api_identifier.clone(),
        };
        let description = match x.description {
            Some(description) => Some(description),
            None => content_model.description.clone(),
        };

        ContentModel::try_new(content_model.id.clone(), name, api_identifier, description)
    });

    let usecase = ContentModelUsecase::new(Arc::new(mock));
    let result = usecase.update(update_content_model).await;

    assert_eq!(result.is_ok(), true);
}

#[tokio::test]
#[rstest::rstest]
async fn delete_success() {
    let mut mock = MockContentModelRepository::new();
    mock.expect_delete().returning(|_| Ok(()));
    let usecase = ContentModelUsecase::new(Arc::new(mock));
    let result = usecase.delete("id".into()).await;

    assert_eq!(result.is_ok(), true);
}
