use labman_server::core::models::UserRole;

mod testing;

#[test]
fn can_create_user() {
    let mut labman = testing::labman::in_memory();
    let mut user_manager = labman.user();
    let result = user_manager.create("alice", &UserRole::Developer);
    assert!(result.is_ok());
}

#[test]
fn cannot_create_duplicate_user() {
    let mut labman = testing::labman::in_memory();
    let mut user_manager = labman.user();
    user_manager.create("bob", &UserRole::Developer).unwrap();
    let result = user_manager.create("bob", &UserRole::Administrator);
    assert!(result.is_err());
}

#[test]
fn can_find_user() {
    let mut labman = testing::labman::in_memory();
    let mut user_manager = labman.user();
    user_manager.create("carol", &UserRole::Developer).unwrap();
    let user = user_manager.get("carol");
    assert!(user.is_ok());
}

#[test]
fn cannot_find_nonexistent_user() {
    let mut labman = testing::labman::in_memory();
    let mut user_manager = labman.user();
    let user = user_manager.get("nonexistent");
    assert!(user.is_err());
}

#[test]
fn can_create_multiple_users() {
    let mut labman = testing::labman::in_memory();
    let mut user_manager = labman.user();
    let dave = user_manager.create("dave", &UserRole::Developer).unwrap();
    let jimmy = user_manager.create("jimmy", &UserRole::Developer).unwrap();
    let eve = user_manager
        .create("eve", &UserRole::Administrator)
        .unwrap();
    assert!(eve.role > dave.role);
    assert!(jimmy.role == dave.role);
}
