use crate::model::user::{Admin, Member, User};

use shared::logger::logger_init_info;

fn before_each() {
    logger_init_info();
}

#[tracing::instrument]
#[rstest::rstest]
#[case::all_fill("a".to_string().repeat(50), "a".to_string().repeat(50), "https://example.com".to_string())]
#[case::without("a".to_string().repeat(50), "a".to_string().repeat(50), "https://example.com".into())]
fn user_new_success(#[case] id: String, #[case] name: String, #[case] icon_url: String) {
    before_each();
    let result = User::<Admin>::try_new(id, name, icon_url);
    assert_eq!(result.is_ok(), true);

    let user = result.unwrap();

    tracing::info!("{:?}", user.permissions().get("contents"));
}
