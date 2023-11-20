use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub store_id: Option<i32>,
  pub branch_id: Option<i32>,
  pub login_session: String
}