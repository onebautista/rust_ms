/* #[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub store_id: Option<i32>,
    pub branch_id: Option<i32>
} */

use schema::schema::users::{self, dsl::*};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub store_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub login_session: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub store_id: Option<i32>,
    pub branch_id: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
    //pub permissions: Option<Vec<String>>
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}