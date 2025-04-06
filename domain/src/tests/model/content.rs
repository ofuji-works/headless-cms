use crate::model::content::{
    Content, ContentCategory, ContentStatus, ContentTag, CreatedBy, UpdatedBy,
};

#[rstest::rstest]
#[case::all_fill("a".repeat(50))]
fn content_new_success(#[case] title: String) {
    let id = uuid::Uuid::now_v7();
    let category = ContentCategory::new("id".into(), "sample".into());
    let status = ContentStatus::Draft;
    let fields = serde_json::from_str("{\"body\": \"Hello World\"}").unwrap();
    let tags = vec![ContentTag::new("id".into(), "sample".into())];
    let created_by = CreatedBy::new("id".into(), "name".into());
    let updated_by = UpdatedBy::new("id".into(), "name".into());
    let published_at = None;
    let created_at = chrono::Utc::now();
    let updated_at = chrono::Utc::now();

    let content = Content::try_new(
        id,
        title,
        category,
        status,
        fields,
        tags,
        created_by,
        updated_by,
        published_at,
        created_at,
        updated_at,
    );

    assert_eq!(content.is_ok(), true);
    assert_eq!("a".repeat(50), content.unwrap().title);
}

#[rstest::rstest]
#[case::all_fill("a".repeat(51))]
fn content_new_failure(#[case] title: String) {
    let id = uuid::Uuid::now_v7();
    let category = ContentCategory::new("id".into(), "sample".into());
    let status = ContentStatus::Draft;
    let fields = serde_json::from_str("{\"body\": \"Hello World\"}").unwrap();
    let tags = vec![ContentTag::new("id".into(), "sample".into())];
    let created_by = CreatedBy::new("id".into(), "name".into());
    let updated_by = UpdatedBy::new("id".into(), "name".into());
    let published_at = None;
    let created_at = chrono::Utc::now();
    let updated_at = chrono::Utc::now();

    let content = Content::try_new(
        id,
        title,
        category,
        status,
        fields,
        tags,
        created_by,
        updated_by,
        published_at,
        created_at,
        updated_at,
    );

    assert_eq!(content.is_err(), true);
}
