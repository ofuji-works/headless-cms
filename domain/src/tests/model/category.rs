use crate::model::category::Category;

#[rstest::rstest]
#[case::all_fill("a".to_string().repeat(50), "a".to_string().repeat(64), Some("a".to_string().repeat(500)))]
#[case::without("a".to_string().repeat(50), "a".to_string().repeat(64), None)]
fn category_new_success(
    #[case] name: String,
    #[case] api_identifier: String,
    #[case] description: Option<String>,
) {
    let result = Category::try_new("id".into(), name, api_identifier, description);

    assert_eq!(result.is_ok(), true);
}

#[rstest::rstest]
#[case::invalid_name("a".to_string().repeat(51), "a".to_string().repeat(64), Some("a".to_string().repeat(500)))]
#[case::invalid_api_identifier("a".to_string().repeat(50), "a".to_string().repeat(65), Some("a".to_string().repeat(500)))]
#[case::invalid_description("a".to_string().repeat(50), "a".to_string().repeat(64), Some("a".to_string().repeat(501)))]
fn category_new_failure(
    #[case] name: String,
    #[case] api_identifier: String,
    #[case] description: Option<String>,
) {
    let result = Category::try_new("id".into(), name, api_identifier, description);

    assert_eq!(result.is_err(), true);
}
