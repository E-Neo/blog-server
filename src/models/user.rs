use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub salt: String,
    pub hash: String,
}
