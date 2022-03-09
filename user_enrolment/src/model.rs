use diesel::{r2d2::ConnectionManager,PgConnection};
use serde::{Serialize,Deserialize};
use uuid::Uuid;

use super::schema::*;

//type alias to avoid long line of code in the project
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

//database model
#[derive(Debug,Serialize,Deserialize,Queryable,Insertable)]
#[table_name = "confirmations"]
pub struct Confirmation{
    pub id: Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize,Queryable,Insertable)]
#[table_name = "users"]
pub struct Users{
    pub id: Uuid,
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct SessionUser{
    pub id: Uuid,
    pub email: String,
}

// any type that implements Into<String> can be used to create a Confirmation
impl<T> From<T> for Confirmation where
T: Into<String> {
     fn from(email: T) -> Self {
        Confirmation {
            id: Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}impl From<Users> for SessionUser {
    fn from(Users { email, id, .. }: Users) -> Self {
        SessionUser { email, id }
    }
}impl Users {
    pub fn from<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        Users {
            id: Uuid::new_v4(),
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

