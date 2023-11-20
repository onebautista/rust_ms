use crate::user::entity::User;
use crate::user::grpc_user::User as UserMessage;

impl From<UserMessage> for User {
  fn from(value: UserMessage) -> Self { 
    Self { 
      id: value.id, 
      username: value.username,
      email: value.email,
      password: value.password, 
      store_id: value.store_id, 
      branch_id: value.branch_id,
      login_session: value.login_session
    }
  }
}

impl From<User> for UserMessage {
  fn from(value: User) -> Self {
      Self {
        id: value.id, 
        username: value.username,
        email: value.email,
        password: value.password, 
        store_id: value.store_id, 
        branch_id: value.branch_id,
        login_session: value.login_session
      }
  }
}