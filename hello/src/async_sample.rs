use futures::executor;

pub struct User {}

pub struct UserId(u32);

pub struct Db {}

impl Db {
    async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {}
}

pub async fn find_by_user_id(db: Db, user_id: UserId) -> Option<User> {
    db.find_by_user_id(user_id).await
}
