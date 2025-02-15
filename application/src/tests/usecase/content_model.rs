use std::sync::Arc;

use domain::{
    model::category::Category,
    repository::category::{CreateCategory, MockCategoryRepository, UpdateCategory},
};

use crate::usecase::category::CategoryUsecase;

#[rstest::fixture]
fn category() -> Category {
    Category::try_new("id".into(), "name".into(), "api_identifier".into(), None).unwrap()
}

#[tokio::test]
#[rstest::rstest]
async fn get_success(category: Category) {
    let mut mock = MockCategoryRepository::new();
    mock.expect_get()
        .returning(move || Ok(vec![category.clone()]));
    let usecase = CategoryUsecase::new(Arc::new(mock));
    let result = usecase.get().await;

    assert_eq!(result.is_ok(), true);
}

#[tokio::test]
#[rstest::rstest]
#[case::simple(CreateCategory::new("name".into(), "api_identifier".into(), None)
)]
async fn create_success(#[case] create_category: CreateCategory) {
    let mut mock = MockCategoryRepository::new();
    mock.expect_create().returning(move |x| {
        Category::try_new("id".into(), x.name, x.api_identifier, x.description)
    });
    let usecase = CategoryUsecase::new(Arc::new(mock));
    let result = usecase.create(create_category).await;

    assert_eq!(result.is_ok(), true);
}

#[tokio::test]
#[rstest::rstest]
#[case::simple(UpdateCategory::new("id".into(), Some("new_name".into()), Some("new_api_identifier".into()), None))]
async fn update_success(category: Category, #[case] update_category: UpdateCategory) {
    let mut mock = MockCategoryRepository::new();
    mock.expect_update().returning(move |x| {
        let name = match x.name {
            Some(name) => name,
            None => category.name.clone(),
        };
        let api_identifier = match x.api_identifier {
            Some(api_identifier) => api_identifier,
            None => category.api_identifier.clone(),
        };
        let description = match x.description {
            Some(description) => Some(description),
            None => category.description.clone(),
        };

        Category::try_new(category.id.clone(), name, api_identifier, description)
    });

    let usecase = CategoryUsecase::new(Arc::new(mock));
    let result = usecase.update(update_category).await;

    assert_eq!(result.is_ok(), true);
}

#[tokio::test]
#[rstest::rstest]
async fn delete_success() {
    let mut mock = MockCategoryRepository::new();
    mock.expect_delete().returning(|_| Ok(()));
    let usecase = CategoryUsecase::new(Arc::new(mock));
    let result = usecase.delete("id".into()).await;

    assert_eq!(result.is_ok(), true);
}
