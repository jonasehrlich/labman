use labman_server::core::entity::user::UserRole;

mod testing;

#[tokio::test]
async fn can_create_user() {
    let labman = testing::labman::in_memory().await;
    let user_manager = labman.user();

    assert!(
        user_manager
            .create("alice", &UserRole::Developer)
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn cannot_create_duplicate_user() {
    let labman = testing::labman::in_memory().await;
    let user_manager = labman.user();
    assert!(
        user_manager
            .create("bob", &UserRole::Developer)
            .await
            .is_ok()
    );

    assert!(
        user_manager
            .create("bob", &UserRole::Administrator)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn can_get_user() {
    let labman = testing::labman::in_memory().await;
    let user_manager = labman.user();
    assert!(
        user_manager
            .create("carol", &UserRole::Developer)
            .await
            .is_ok()
    );
    let user = user_manager.get_by_name("carol").await;
    assert!(user.is_ok());

    assert!(
        user_manager
            .get_by_id(user.unwrap().unwrap().id)
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn cannot_get_nonexistent_user() {
    let labman = testing::labman::in_memory().await;
    assert!(
        labman
            .user()
            .get_by_name("nonexistent")
            .await
            .unwrap()
            .is_none()
    );
    assert!(labman.user().get_by_id(123).await.unwrap().is_none());
}

#[tokio::test]
async fn can_create_multiple_users() {
    let labman = testing::labman::in_memory().await;
    let user_manager = labman.user();
    let dave = user_manager
        .create("dave", &UserRole::Developer)
        .await
        .unwrap();
    let jimmy = user_manager
        .create("jimmy", &UserRole::Developer)
        .await
        .unwrap();
    let eve = user_manager
        .create("eve", &UserRole::Administrator)
        .await
        .unwrap();
    assert!(eve.role > dave.role);
    assert!(jimmy.role == dave.role);
}

#[tokio::test]
async fn can_delete_user() {
    let labman = testing::labman::in_memory().await;
    let user_manager = labman.user();
    let frank = user_manager
        .create("frank", &UserRole::Developer)
        .await
        .unwrap();
    assert!(user_manager.delete(frank.id).await.is_ok());
    assert!(user_manager.delete(frank.id).await.is_err());
    assert!(user_manager.get_by_id(frank.id).await.unwrap().is_none());
}
